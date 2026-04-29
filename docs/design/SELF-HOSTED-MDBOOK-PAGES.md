# Self-hosted mdBook docs on Forgejo + k8s

## Intent

The seqlings repo migrated from GitHub to a self-hosted Forgejo instance (`git.navicore.tech`). The old `docs.yml` workflow used `actions/configure-pages` / `actions/upload-pages-artifact` / `actions/deploy-pages` — all of which call GitHub APIs that don't exist on Forgejo. Forgejo has no built-in Pages equivalent and no official "forgejo-pages" helper action. The workflow has been deleted; the `book.toml`, `docs/` source, and `scripts/generate-docs.sh` are still in the repo, ready to drive a new pipeline.

We want the docs site back, served from the homelab k8s cluster (already running Forgejo + Forgejo runner under Flux in `../k8s-vcluster-homelab`), at a stable URL. The pipeline should fit the patterns already in use there: workflow → image → registry → Flux → pod.

## Constraints

- **No GitHub-specific actions.** Anything that talks to `api.github.com/pages/*` is out.
- **Stay inside the existing homelab.** No external Pages provider (Cloudflare Pages, Netlify, etc.) — the point is self-hosted.
- **Reuse existing infra patterns.** Flux + image automation already deploys other apps in `k8s-vcluster-homelab/apps/`. The docs server should be just another app there, not a new infra category.
- **Out of scope:** PR previews, multi-version docs, search beyond mdBook's built-in, auth/private docs.
- **Out of scope:** rewriting `book.toml` or `scripts/generate-docs.sh` — they work; only the deploy half is broken.

## Approach

Two-part change, one in each repo:

**In seqlings (`.forgejo/workflows/docs.yml` or `.github/workflows/docs.yml`):**
1. On push to `main` touching `docs/**`, `book.toml`, `README.md`, or the workflow itself, run `mdbook build` (after `scripts/generate-docs.sh`) → produces `./book/`.
2. Build a tiny nginx image with `./book/` baked into `/usr/share/nginx/html`. Tag with the short commit SHA.
3. Push to the in-cluster registry (whatever Forgejo's container registry endpoint is — confirm during implementation).

**In k8s-vcluster-homelab (`apps/seqlings-docs/`):**
1. Deployment running the nginx image, single replica.
2. Service + Ingress (or whatever ingress pattern the other apps use — match `apps/forgejo/` conventions).
3. Flux image-automation entry tracking the new image tag, same way other apps in the repo are tracked.

The nginx-with-baked-content approach is preferred over a git-sync sidecar because (a) it matches the image-driven Flux pattern already in place, (b) rollbacks are just `kubectl rollout undo`, (c) no runtime git credentials.

## Domain Events

- **Input**: `DocsSourceChanged { paths: [...] }` — push to main touching tracked paths.
- **Workflow**: `DocsBuilt { sha, image_tag }` → `DocsImagePushed { registry, tag }`.
- **Flux**: `ImageUpdateAutomation` notices new tag → commits manifest bump → `Kustomization` reconciles → `Deployment` rolls.
- **Side effect**: live site at the chosen hostname now serves content from the new commit.
- **Failure modes worth naming**:
  - Workflow build fails → no new image, site stays on old version (good).
  - Image push fails → workflow fails loudly (good).
  - Flux reconcile fails → site stays on old version, Flux alerts (good).
  - mdbook silently producing empty/broken output → site goes blank. **Add a smoke check** in the workflow (e.g. assert `./book/index.html` exists and is >1KB) before building the image.

## Checkpoints

1. **Local build still works.** `./scripts/generate-docs.sh && mdbook build` produces `./book/index.html`. (Sanity — no infra involved.)
2. **Workflow fires and pushes an image.** Push a no-op docs change to main; runner builds, image lands in registry with expected tag.
3. **Flux picks up the tag.** `flux get image update -A` shows the new tag committed to the homelab repo within the reconcile interval.
4. **Site serves.** `curl https://<chosen-host>/` returns the index page; navigation links resolve.
5. **Rollback works.** `kubectl rollout undo deployment/seqlings-docs -n <ns>` restores the prior image; site reflects old content.
6. **Path filter works.** Touching a non-docs file in the repo does *not* trigger a docs rebuild.

## Open questions to resolve at implementation time

- Which container registry — Forgejo's built-in package registry, or a separate one already in the homelab?
- Hostname — subdomain of `navicore.tech`, path under `git.navicore.tech`, or something else?
- Does the existing ingress controller terminate TLS for arbitrary new hosts, or does each app need a cert annotation?
- Do we want the workflow to also run on tag pushes (so docs can be pinned to releases), or main-only?

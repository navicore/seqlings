# Track B — Hint Pedagogy Findings

Working ledger for the human/judgmental hint audit. Methodology and
rubric: see `SOLUTIONS_AND_HINTS_AUDIT.md`.

## Rubric (short form)

- **Teaches** — hint scaffolds understanding (names primitives,
  describes shape, traces state, explains *why*). Showing the full
  word body is fine when accompanied by enough "here's why" prose
  that the reader couldn't just delete the prose and copy-paste.
- **Leaks** — hint dumps a complete solution body with little or no
  explanation. If you delete every non-code line and still have a
  working answer, that's the smell.
- **Borderline** — hint shows the answer with *some* explanation
  but feels thin. Rewrite if convenient, leave if not.
- **N/A** — solution body is sub-3-token (e.g. `0 variant.field-at`)
  and the hint can't avoid showing it. Carve-out from the rubric.

Verdicts are case-sensitive in this ledger so they grep cleanly.

## Summary

194 hint files audited.

| Verdict     | Count | Share |
|-------------|-------|-------|
| Teaches     | 136   | 70%   |
| Leaks       | 0     | 0%    |
| Borderline  | 26    | 13%   |
| N/A         | 32    | 16%   |

**All `Leaks` rewritten.** Track B chapter-by-chapter sweep complete:

- ✓ ch 08 (words) — 4 rewrites + 2 reclassified
- ✓ ch 09 (recursion) — 6 rewrites incl. 2 borderlines
- ✓ ch 11 (types) — 6 rewrites incl. 2 borderlines
- ✓ ch 13 (strings) — 5 rewrites incl. 2 misleading-hints
- ✓ ch 16 (maps) — 6 rewrites incl. 2 borderlines
- ✓ ch 28 (std-fmath) — all 5 rewrites
- ✓ Final batch sweep — 13 scattered leaks across ch 03, 05, 10, 12, 24, 25, 27, 31, 36.

**Borderline (29 remaining)** were not rewritten in this pass — they show the answer but with enough explanation to not be egregious. Candidates for future polish if any one of them keeps tripping up learners; not blocking.

**N/A (32)** are hints for words whose entire body is 1–2 tokens — the rubric's carve-out, can't be helped without going around in circles.

**Chapters concentrating `Borderline`** (rewrite if convenient):
- ch 31 (regex) — 4 of 6
- ch 36 (amdahls-law) — 3 of 5
- ch 04 (floats) — 3 of 5

**Clean chapters** (no Leaks, no Borderline): 06 booleans, 07 conditionals,
14 variants (1 borderline), 18 bitwise, 19 io, 23 time, 26 tcp, 29 weave,
32 compression, 33 crypto, 34 http-client, 35 littles-law, 38 cons-list.

## Findings

One row per hint file. Audit by chapter.

| Hint                                              | Verdict    | Notes |
|---------------------------------------------------|------------|-------|
| hints/00-intro/01-hello.md                        | Teaches    | explains literal push then io.write-line consume |
| hints/00-intro/02-comments.md                     | Teaches    | explains # syntax and shell-script tradition |
| hints/00-intro/03-numbers.md                      | Teaches    | explains literals, types, assert-eq mechanics |
| hints/01-stack-basics/01-push.md                  | Teaches    | explains stack notation and pop order before answer |
| hints/01-stack-basics/02-dup.md                   | N/A        | body is `7 dup` |
| hints/01-stack-basics/03-drop.md                  | N/A        | body is `drop` |
| hints/01-stack-basics/04-swap.md                  | N/A        | body is `swap` |
| hints/01-stack-basics/05-over.md                  | N/A        | body is `over` |
| hints/01-stack-basics/06-rot.md                   | N/A        | body is `rot` (sidebar example uses wrong stack shape) |
| hints/02-stack-advanced/01-nip.md                 | N/A        | body is `nip` with decomposition prose |
| hints/02-stack-advanced/02-tuck.md                | N/A        | body is `tuck` with decomposition prose |
| hints/02-stack-advanced/03-2dup.md                | N/A        | body is `2dup`, contrasts with dup dup |
| hints/02-stack-advanced/04-3drop.md               | N/A        | body is `3drop` |
| hints/02-stack-advanced/05-pick.md                | Borderline | reveals depth `2` directly; could ask learner to count |
| hints/02-stack-advanced/06-roll.md                | Teaches    | bug fix: hint said `4 roll`, solution is `3 roll`; rewritten to teach 0-indexed depth |
| hints/02-stack-advanced/07-aux.md                 | Teaches    | annotated per-token trace with rationale |
| hints/03-arithmetic/01-add.md                     | Borderline | shows full body; transformation prose is generic |
| hints/03-arithmetic/02-subtract.md                | Teaches    | explains operand order with minuend/subtrahend why |
| hints/03-arithmetic/03-multiply.md                | Teaches    | rewritten: explains pattern + i. prefix family, no body shown |
| hints/03-arithmetic/04-divide.md                  | Borderline | shows body; truncation context adds some why |
| hints/03-arithmetic/05-combine.md                 | Teaches    | per-token stack trace |
| hints/03-arithmetic/06-expressions.md             | Teaches    | strategy then full stack trace |
| hints/04-floats/01-float-literals.md              | N/A        | body is `2.5` |
| hints/04-floats/02-f-add.md                       | Borderline | shows full body; explicit-types prose not load-bearing |
| hints/04-floats/03-f-multiply.md                  | Borderline | shows body; contrasts int vs float division |
| hints/04-floats/04-f-compare.md                   | Borderline | shows body; equality caveat unrelated to picking `>` |
| hints/04-floats/05-mixed.md                       | Teaches    | step-by-step explaining the int->float conversion |
| hints/05-comparison/01-equals.md                  | Teaches    | rewritten: explains stack effect, `=` vs `i.=`, no body |
| hints/05-comparison/02-less-greater.md            | Borderline | shows body; operand-order explanation helps |
| hints/05-comparison/03-not-equal.md               | Teaches    | rewritten: stack effect, <> history, `i.= not` alternative |
| hints/05-comparison/04-less-equal.md              | Borderline | shows body; boundary-check sidebar is generic |
| hints/05-comparison/05-chaining.md                | Teaches    | per-token stack trace of 7-token answer |
| hints/06-boolean/01-and.md                        | N/A        | body is `true true and`, trivially shaped |
| hints/06-boolean/02-or.md                         | N/A        | body is `false true or`, trivially shaped |
| hints/06-boolean/03-not.md                        | N/A        | body is `false not` |
| hints/06-boolean/04-combine.md                    | Teaches    | traces each step of the 5-token expression |
| hints/07-conditionals/01-if.md                    | Teaches    | explains stack order of bool+quots, branch shape match |
| hints/07-conditionals/02-when.md                  | Teaches    | explains shape constraint, library origin, no body shown |
| hints/07-conditionals/03-nested.md                | Teaches    | explains pattern and dup-compare-if idiom, no body shown |
| hints/07-conditionals/04-fizzbuzz.md              | Teaches    | explains ordering trap and divisibility primitives, no body |
| hints/08-words/01-define.md                       | N/A        | body is `3 i.*` (2 tokens; reclassified from Leaks) |
| hints/08-words/02-stack-effects.md                | Teaches    | rewritten: explains 2dup motivation, assert order trap |
| hints/08-words/03-calling.md                      | N/A        | body is `square square` (2 tokens; reclassified from Leaks) |
| hints/08-words/04-refactor.md                     | Teaches    | rewritten: decomposes pattern, lists alternatives without dumping |
| hints/08-words/05-helper-words.md                 | Teaches    | rewritten: explains i.modulo Bool trap, names the three pieces |
| hints/08-words/06-documentation.md                | Teaches    | rewritten: helper-first strategy, `when` semantics, no bodies |
| hints/09-recursion/01-countdown.md                | Teaches    | rewritten: names the four blanks in the recursion template, explains dup placement |
| hints/09-recursion/02-factorial.md                | Teaches    | rewritten: frames as pattern transfer from countdown |
| hints/09-recursion/03-fibonacci.md                | Teaches    | rewritten: explains the no-op base case and inter-call swap without dumping body |
| hints/09-recursion/04-accumulator.md              | Teaches    | rewritten: names the transformation, describes shape without literal body |
| hints/09-recursion/05-mutual.md                   | Teaches    | rewritten: parity-flip explanation, base cases described not shown |
| hints/09-recursion/06-tco.md                      | Teaches    | rewritten: emphasizes "nothing after recurse", names `2dup`/`nip` without dumping body |
| hints/10-quotations/01-basics.md                  | Teaches    | rewritten: explains brackets-as-data without dumping body |
| hints/10-quotations/02-call.md                    | Teaches    | rewritten: explains higher-order semantics, body in prose |
| hints/10-quotations/03-higher-order.md            | Teaches    | hints at stack setup without showing tuck/call/swap/call body |
| hints/11-types/01-predicates.md                   | Teaches    | rewritten: frames as prefix-family naming convention |
| hints/11-types/02-int-ops.md                      | Teaches    | rewritten: centers the Bool-flag trap on i./ and i.modulo |
| hints/11-types/03-float-ops.md                    | Teaches    | rewritten: explains f.-prefix family and the no-Bool difference |
| hints/11-types/04-string-type.md                  | Teaches    | rewritten: lists the vocabulary and `?` predicate convention |
| hints/11-types/05-list-type.md                    | Teaches    | rewritten: lists-are-variants insight, no bodies dumped |
| hints/11-types/06-row-polymorphism.md             | Teaches    | rewritten: prose-described dance of the four primitives |
| hints/12-type-conversions/01-int-to-string.md     | N/A        | body is `2024 int->string` (2 tokens, fully determined) |
| hints/12-type-conversions/02-string-to-int.md     | Teaches    | shows pipeline but explains drop, success flag, robust pattern |
| hints/12-type-conversions/03-int-float.md         | Teaches    | rewritten: explains both conversions and toward-zero truncation |
| hints/12-type-conversions/04-float-string.md      | Teaches    | bug fix: hint was missing the `drop` after `string->float`; rewritten with Bool-trap framing |
| hints/12-type-conversions/05-combine.md           | Teaches    | rewritten: emphasizes the Bool-drop trap, also fixed for test-round-trip |
| hints/13-strings/01-concat.md                     | N/A        | body fully determined by lesson |
| hints/13-strings/02-length.md                     | Teaches    | rewritten: names the two pieces, no body shown |
| hints/13-strings/03-char-at.md                    | Borderline | full body but one-line note about Int return adds a real warning |
| hints/13-strings/04-substring.md                  | N/A        | body is 3 tokens fully determined |
| hints/13-strings/05-find-contains.md              | Teaches    | rewritten: was misleading ("already correct"); now names recipe |
| hints/13-strings/06-split.md                      | Teaches    | rewritten: prose-only walk, points at next chapter for lists |
| hints/13-strings/07-transform.md                  | Teaches    | rewritten: was misleading; now matches each transform to its test |
| hints/13-strings/08-compare.md                    | Teaches    | rewritten: lowercase-both recipe described, no literal body |
| hints/14-variants/01-basics.md                    | Teaches    | shows worked example for `is-stop?`, learner adapts for `is-go?` |
| hints/14-variants/02-option.md                    | Teaches    | explains `{ >value }` extraction with worked example |
| hints/14-variants/03-result.md                    | Teaches    | explains constructors and match shape; doesn't dump full body |
| hints/14-variants/04-match.md                     | Teaches    | traces stack through swap/nip in both branches |
| hints/14-variants/05-combine.md                   | Borderline | full body shown, explains "why check first" but minimal trace |
| hints/15-lists/01-basics.md                       | N/A        | body is `list.length 3 i.=`, shape determined by lesson |
| hints/15-lists/02-map.md                          | Teaches    | shows analogous `double-all`, not the literal `upcase-all` solution |
| hints/15-lists/03-filter.md                       | Borderline | shows full predicate body; pattern leaks even if predicate differs |
| hints/15-lists/04-fold.md                         | Teaches    | uses `sum-list` example; solution is different `join-with-dash` |
| hints/15-lists/05-combine.md                      | Borderline | shows analogous pipeline; same shape as solution |
| hints/16-maps/01-basics.md                        | Teaches    | rewritten: explains map.set's signature and chain pattern |
| hints/16-maps/02-get.md                           | Teaches    | rewritten: explains the two-value return and why no sentinel |
| hints/16-maps/03-get-safe.md                      | Teaches    | rewritten: stack-state walk through both branches, no body block |
| hints/16-maps/04-remove.md                        | Teaches    | rewritten: per-test recipe, Bool-assertion vocabulary |
| hints/16-maps/05-inspection.md                    | Teaches    | rewritten: 4-word menu + per-test mapping, no bodies |
| hints/16-maps/06-combine.md                       | Teaches    | rewritten: kept the rot-trace insight, removed literal body block |
| hints/18-bitwise/01-and-or.md                     | Teaches    | explains masking and bit-set semantics, no code dumped |
| hints/18-bitwise/02-xor.md                        | Teaches    | explains XOR cancellation property, no code dumped |
| hints/18-bitwise/03-not.md                        | Teaches    | explains two's complement and self-inverse, no code dumped |
| hints/18-bitwise/04-shifts.md                     | Teaches    | explains shl/shr as multiply/divide by 2^N, no code dumped |
| hints/18-bitwise/05-bit-flags.md                  | Teaches    | explains semantics of each helper, no code body shown |
| hints/19-io/01-write-line.md                      | N/A        | body is `"Seq is fun!" io.write-line` (2 tokens) |
| hints/19-io/02-read-line.md                       | Teaches    | describes blocking and newline-stripping semantics |
| hints/19-io/03-read-status.md                     | Teaches    | explains return shape and standard EOF pattern |
| hints/19-io/04-read-n.md                          | Teaches    | explains use case for fixed-size reads |
| hints/19-io/05-interactive.md                     | Teaches    | explains write/read-line pattern for inline prompts |
| hints/20-files/01-read.md                         | Teaches    | explains dual return, fixture newline, chomp vs trim |
| hints/20-files/02-write.md                        | Teaches    | explains arg order mnemonic, overwrite semantics, chomp rationale |
| hints/20-files/03-exists.md                       | Teaches    | explains semantics and when not to use vs file.slurp |
| hints/20-files/04-lines.md                        | Borderline | full body shown; good prose on polymorphism but body dumped |
| hints/20-files/05-combine.md                      | Teaches    | shows spit/append pattern with rationale; same shape as solution |
| hints/21-args/01-basics.md                        | N/A        | body is `args.count 1 i.-`, fully determined by lesson |
| hints/21-args/02-at.md                            | N/A        | body is `args.at string.empty?` (2 tokens) |
| hints/21-args/03-safe-access.md                   | Teaches    | full body shown with detailed stack trace and `over` rationale |
| hints/21-args/04-combine.md                       | Borderline | full multi-branch body shown; dispatch explanation is thin |
| hints/22-os/01-getenv.md                          | Teaches    | explains return shape and branch semantics before showing body |
| hints/22-os/02-paths.md                           | Borderline | full nested-if body shown; only one sentence of "why dup" prose |
| hints/22-os/03-cwd.md                             | Teaches    | leans on prior 01-getenv shape, explains reuse |
| hints/22-os/04-combine.md                         | Teaches    | walks through stack state after os.home-dir before showing body |

| hints/23-time/01-now.md                           | Teaches    | explains stack order and notes i.> vs bare > |
| hints/23-time/02-sleep.md                         | Teaches    | names sleep-ms effect and why true is pushed |
| hints/23-time/03-measure.md                       | Teaches    | explains why no swap is needed before i.<= |
| hints/23-time/04-combine.md                       | Teaches    | full stack trace and explains i.- argument order |
| hints/24-channels/01-create.md                    | Teaches    | rewritten: explains the dup-for-each-op and success Bool |
| hints/24-channels/02-send-receive.md              | Teaches    | rewritten: FIFO + dup pattern + why-queue-not-stack |
| hints/24-channels/03-safe-ops.md                  | Teaches    | explains return-status semantics for close cases |
| hints/24-channels/04-close.md                     | Teaches    | explains close semantics and post-close behavior |
| hints/24-channels/05-patterns.md                  | Teaches    | traces dup/over/rot, explains hardcoding vs recursion |
| hints/25-spawn/01-basics.md                       | Teaches    | traces stack at marker, explains receive return shape |
| hints/25-spawn/02-return-value.md                 | Teaches    | explains closure capture, why no dup needed |
| hints/25-spawn/03-communication.md                | Teaches    | traces both swaps and closure capture of both chans |
| hints/25-spawn/04-ping-pong.md                    | Teaches    | rewritten: 3-step recipe in prose, two-channel rationale |
| hints/25-spawn/05-worker-pool.md                  | Borderline | shows full pattern with stack comments but little why |
| hints/26-tcp/01-listen.md                         | Teaches    | explains net.tcp.* Bool pattern and port-reuse reason |
| hints/26-tcp/02-accept.md                         | Teaches    | deep cooperative-scheduling explanation, ownership rules |
| hints/26-tcp/03-read-write.md                     | Teaches    | explains dup-before-read ownership and write-Bool lint |
| hints/26-tcp/04-close.md                          | Teaches    | explains three-way read outcome and TCP loop bug |
| hints/26-tcp/05-echo.md                           | Teaches    | role split, ownership rules, chapter composition |
| hints/27-std-imath/01-abs.md                      | N/A        | body is `i.- abs` |
| hints/27-std-imath/02-min-max.md                  | Teaches    | rewritten: over-over preservation + rot rot recovery in prose |
| hints/27-std-imath/03-clamp.md                    | N/A        | body is `0 100 clamp` |
| hints/27-std-imath/04-gcd.md                      | Teaches    | annotates each stack step with purpose |
| hints/27-std-imath/05-power.md                    | Teaches    | explains arg order, why drop the success Bool |
| hints/27-std-imath/06-combine.md                  | Teaches    | explains rot semantics and aux-stack idiom |
| hints/28-std-fmath/01-sqrt.md                     | Teaches    | rewritten: decomposes Pythagoras into 3 steps, dup-f.* idiom note |
| hints/28-std-fmath/02-trig.md                     | Teaches    | rewritten: per-test recipe, radians vs degrees aside |
| hints/28-std-fmath/03-exp-log.md                  | Teaches    | rewritten: arg-order trap + full builtin menu |
| hints/28-std-fmath/04-round.md                    | Teaches    | rewritten: 4-mode menu + per-test match + banker's-rounding note |
| hints/28-std-fmath/05-combine.md                  | Teaches    | rewritten: prose-described composition, no literal bodies |
| hints/29-weave/01-basics.md                       | Teaches    | step-by-step caller/weave protocol walkthrough |
| hints/29-weave/02-multiple-yields.md              | Teaches    | full yield-cycle diagram and explains drop-after-yield |
| hints/29-weave/03-accumulator.md                  | Teaches    | detailed stack trace and TCO/cancel rationale |
| hints/29-weave/04-cancellation.md                 | Teaches    | explains cancel vs resume-to-completion tradeoffs |
| hints/30-encoding/01-base64.md                    | Teaches    | explains decode's success-Bool return shape |
| hints/30-encoding/02-base64url.md                 | Borderline | brief charset note then full wrapper bodies |
| hints/30-encoding/03-hex.md                       | Borderline | one-line decode-failure note then full wrappers |
| hints/30-encoding/04-roundtrip.md                 | Teaches    | explains why no plumbing needed between encode/decode |
| hints/30-encoding/05-errors.md                    | Teaches    | explains if-quotation pattern and empty success branch |
| hints/31-regex/01-match.md                        | Borderline | full body shown; pattern notes explain but body otherwise dumped |
| hints/31-regex/02-find.md                         | Teaches    | rewritten: two return-shape distinction + backslash escaping note |
| hints/31-regex/03-replace.md                      | Borderline | full body shown; pattern notes thin but explain replacements |
| hints/31-regex/04-captures.md                     | Teaches    | explains list.get/dup-over-rot reasoning and capture ordering |
| hints/31-regex/05-split.md                        | Borderline | full body; pattern + gotcha notes thin |
| hints/31-regex/06-validate.md                     | Borderline | full body; per-pattern breakdown explains anchoring idea |
| hints/32-compression/01-gzip.md                   | N/A        | bodies are single-builtin wraps |
| hints/32-compression/02-zstd.md                   | N/A        | bodies are single-builtin wraps |
| hints/32-compression/03-levels.md                 | N/A        | bodies are 2-token `N compress.gzip-level` style |
| hints/32-compression/04-roundtrip.md              | Teaches    | explains why naive chain fails and how `if` collapses to one shape |
| hints/33-crypto/01-sha256.md                      | Teaches    | traces stack through swap/= and explains determinism |
| hints/33-crypto/02-hmac.md                        | Teaches    | explains rot rot ordering and constant-time rationale |
| hints/33-crypto/03-random.md                      | N/A        | single-builtin passthroughs |
| hints/33-crypto/04-encrypt.md                     | N/A        | bodies are single-builtin wraps; GCM prose explains design |
| hints/33-crypto/05-pbkdf2.md                      | N/A        | body is `100000 crypto.pbkdf2-sha256`; iterations rationale is good |
| hints/33-crypto/06-signatures.md                  | N/A        | three single-builtin wraps; conventions prose is bonus |
| hints/34-http-client/01-get.md                    | Teaches    | explains dup pattern and branching for response-body |
| hints/34-http-client/02-post.md                   | Teaches    | step-by-step stack trace through concat chain |
| hints/34-http-client/03-errors.md                 | Teaches    | explains range-check pattern and dup-cascade for nested if |
| hints/34-http-client/04-json.md                   | Teaches    | traces api-success? stack and explains json-parse shape match |
| hints/35-littles-law/01-measure-latency.md        | Teaches    | explains time.now units and why drops are needed |
| hints/35-littles-law/02-throughput.md             | Teaches    | explains pick-depth rule and why throughput cap proves overlap |
| hints/35-littles-law/03-queue-depth.md            | Teaches    | explains in-flight counter and pick-depth shifts |
| hints/35-littles-law/04-verify-law.md             | Teaches    | walks each recv line and shows the L=λW algebra |
| hints/35-littles-law/05-capacity-planning.md      | Teaches    | explains K derivation and integer-math scaling trick |
| hints/36-amdahls-law/01-serial-baseline.md        | Teaches    | bug fix: hint juggled `start` on data stack; solution uses aux. Rewritten to match. |
| hints/36-amdahls-law/02-parallel-speedup.md       | Teaches    | rewritten: dup-vs-over distinction, cleanup with nip nip explained |
| hints/36-amdahls-law/03-serial-fraction.md        | Borderline | shows body with brief Amdahl framing line |
| hints/36-amdahls-law/04-calculate-speedup.md      | N/A        | body is `40 10 i./`; brief Amdahl sentence accompanies |
| hints/36-amdahls-law/05-diminishing-returns.md    | Borderline | full body shown; observation about 2x vs 4x explains lesson |
| hints/37-combinators/01-dip.md                    | Teaches    | shows minimal example with inline trace comment, no full dump |
| hints/37-combinators/02-keep.md                   | Teaches    | explains preservation semantics and inner-dup distinction |
| hints/37-combinators/03-bi.md                     | Teaches    | example with order/result explanation |
| hints/37-combinators/04-dip-deeper.md             | Borderline | shows the solution shape with brief trace comment |
| hints/37-combinators/05-keep-chain.md             | Borderline | shows both keep lines with brief cube-math note |
| hints/37-combinators/06-combine.md                | Teaches    | full body shown but with detailed step-by-step trace |
| hints/38-cons-list/01-constructors.md             | Teaches    | explains variant.make-N tagging and ADT dispatch rationale |
| hints/38-cons-list/02-predicates.md               | Teaches    | explains tag mechanics and O(1) reasoning |
| hints/38-cons-list/03-accessors.md                | Teaches    | explains field-at indexing and Empty precondition |
| hints/38-cons-list/04-length.md                   | Teaches    | detailed stack trace and accumulator-pattern rationale |
| hints/38-cons-list/05-reverse.md                  | Teaches    | full stack trace and "reverse for free" insight |
| hints/38-cons-list/06-append.md                   | Teaches    | full trace plus dip-rationale and accumulator-vs-not discussion |

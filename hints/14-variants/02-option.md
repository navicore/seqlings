# Hint: Option Type

## Using match with fields

When a variant has fields, use `{ >fieldname }` to extract them:

```seq
: is-some? ( Option -- Bool )
    match
        Some { >value } -> drop true   # drop the extracted value
        None -> false
    end
;
```

## Solution

```seq
: is-none? ( Option -- Bool )
    match
        Some { >value } -> drop false
        None -> true
    end
;
```

`Some` and `None` are mutually exclusive variants.

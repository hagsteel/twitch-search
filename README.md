# Show list of people currently streaming Rust dev

Prints a list of live streams from "science and technology" that has the search
termin the title.


Usage: 

```
# Searching gamedev
stream-search gamedev

# Searchign for rust (no search term falls back to "rust")
stream-search
```

*Note:* requires two env vars set to a valid OAuth token and client id:
* `TWITCHY_TOKEN`
* `TWITCHY_CLIENT_ID`

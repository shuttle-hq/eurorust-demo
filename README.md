# Shuttle Demo 🚀

This is a demo service for showcasing Shuttle.

The demo consists of 3 steps:

1. Creating a "hello world" endpoint.
2. Adding the shared database.
3. Serving static assets.

## Running the Demo 🏃

First, run [`git-history`](https://github.com/pomber/git-history) in a separate terminal:

```sh
npx git-file-history src/main.rs
```

This will open a tab at http://localhost:5000 for browsing the code changes.

Then check out the selected commit/branch and run/deploy the service:

```sh
git checkout <step>

cargo shuttle run

cargo shuttle deploy
```

| Step | Description   | Demo                                                                                                                                                                                | Commit    | Branch     |
| ---- | ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------- | ---------- |
| 1st  | Hello World   | Visit `http://127.0.0.1:8000` or `curl` it                                                                                                                                          | `1111111` | `1st-step` |
| 2nd  | Shared DB     | 1. Show local database being created with docker<br>2. Show that a database is provisioned on our platform                                                                          | `2222222` | `2nd-step` |
| 3rd  | Static Assets | 1. Show static assets app locally<br>2. Open `http://127.0.0.1:8000`, then add `/assets` to the end of the URL<br>3. Show that you can add and retrieve greetings from the database | `3333333` | `3rd-step` |

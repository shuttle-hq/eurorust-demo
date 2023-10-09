# Shuttle Demo 🚀

This is a demo service for showcasing Shuttle.

The demo consists of two steps:

1. Creating a "hello world" endpoint.
2. Adding the shared database and serving static assets.

## Running the Demo 🏃

First, run [`git-history`](https://github.com/pomber/git-history) in a separate terminal:

```sh
npx git-file-history src/main.rs
```

This will open a tab at http://localhost:5000 for browsing the code changes.

![git-file-history](https://github.com/shuttle-hq/eurorust-demo/assets/24392180/10e83958-8554-4e43-b2fa-4f4153bbe03a)

Then check out the selected branch and run/deploy the service:

```sh
git checkout <step>

cargo shuttle run

cargo shuttle deploy
```

| Step | Description               | Demo                                                                                                                                                                                                                                                                                              | Branch     |
| ---- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| 1st  | Hello World               | Visit `http://127.0.0.1:8000` or `curl` it                                                                                                                                                                                                                                                        | `1st-step` |
| 2nd  | Shared DB & Static Assets | 1. Show local database being created with docker<br>2. Show that a database is provisioned on our platform<br>3. Show static assets app locally<br>4. Open `http://127.0.0.1:8000`, then add `/assets` to the end of the URL<br>5. Show that you can add and retrieve greetings from the database | `2nd-step` |

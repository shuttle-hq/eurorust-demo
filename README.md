# Shuttle Demo 🚀

This is a demo service for showcasing Shuttle.

The demo consists of two steps:

1. Creating a "hello world" endpoint.
2. Adding the shared database and serving static assets.

## Running the Demo 🏃

You can visualize each change easily by going [here](https://github.githistory.xyz/shuttle-hq/eurorust-demo/blob/main/src/main.rs), or by running [`git-history`](https://github.com/pomber/git-history) yourself in a separate terminal:

```sh
npx git-file-history src/main.rs
```

This will open a tab at http://localhost:5000 for browsing the code changes.

![git-file-history](https://github.com/shuttle-hq/eurorust-demo/assets/24392180/10e83958-8554-4e43-b2fa-4f4153bbe03a)

Then first check out the first step:

```sh
git checkout 1st-step
```

If this is your first time deploying the project, you will need the Shuttle CLI: `cargo-shuttle`. 

```sh
cargo install cargo-shuttle
```

Then, for the initial project creation:

```sh
cargo shuttle login

# Edit Cargo.toml to change the name of the project to something unique
# like shuttle-eurorust-demo-your-name

cargo shuttle project start
```

You can then move forward, run/deploy the service, and checkout the next steps. If you had to change the project name, don't forget to do it again after switching branches.

```sh
git checkout <step>

cargo shuttle run

cargo shuttle deploy
```

| Step | Description               | Demo                                                                                                                                                                                                                                                                                              | Branch     |
| ---- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| 1st  | Hello World               | Visit `http://127.0.0.1:8000` or `curl` it                                                                                                                                                                                                                                                        | `1st-step` |
| 2nd  | Shared DB & Static Assets | 1. Show local database being created with docker<br>2. Show that a database is provisioned on our platform<br>3. Show static assets app locally<br>4. Open `http://127.0.0.1:8000`, then add `/assets` to the end of the URL<br>5. Show that you can add and retrieve greetings from the database | `2nd-step` |

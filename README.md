# Schlaugh Bot

A bot that posts my latest schlaupdate to my Discord server.

I computed the schlaupdate to be at 9 AM UTC, so the bot runs at 9:10 AM UTC.

## Configuration

For now, the values are hardcoded, so you'll need to edit the code to change the webhook URL and the user ID.

## Deployment

```bash
npx wrangler deploy
```

## What it looks like

![Screenshot of the bot posting the latest schlaupdate](./screenshot.png)
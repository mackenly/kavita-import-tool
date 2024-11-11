import { test, expect } from "@playwright/test";

test("the headers and footer text are there for the home page", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator("h1")).toHaveText("Kavita Import Tool");
    await expect(page.locator("a:has-text('GitHub Sponsors @mackenly')")).toHaveAttribute("href", "https://github.com/sponsors/mackenly/");
});

test("the headers and footer text are there for the quickstart page", async ({ page }) => {
    await page.goto("/quickstart");
    await expect(page.locator("h1")).toHaveText("Quickstart");
});

test("the headers and footer text are there for the about page", async ({ page }) => {
    await page.goto("/about");
    await expect(page.locator("h1")).toHaveText("About");
});

test("the headers and footer text are there for the notes page", async ({ page }) => {
    await page.goto("/notes");
    await expect(page.locator("h1")).toHaveText("Development Notes");
});
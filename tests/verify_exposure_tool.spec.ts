import { test, expect } from '@playwright/test';

test('Verify Exposure Tool and Language Selector', async ({ page }) => {
  await page.goto('file://' + process.cwd() + '/index.html');

  // Verify Language Selector
  const langSelector = page.locator('div.relative.group:has-text("IT")');
  await expect(langSelector).toBeVisible();

  // Verify Exposure Tool
  const exposureSection = page.locator('#exposure-tool');
  await expect(exposureSection).toBeVisible();
  await exposureSection.scrollIntoViewIfNeeded();
  await page.screenshot({ path: '/home/jules/verification/screenshots/exposure_tool_init.png' });

  // Start Scan
  const startBtn = page.locator('#start-scan-btn');
  await startBtn.click();

  // Wait for progress
  const progress = page.locator('#scanner-progress');
  await expect(progress).toBeVisible();
  await page.waitForTimeout(2000); // Progressing...
  await page.screenshot({ path: '/home/jules/verification/screenshots/exposure_tool_progress.png' });

  // Wait for results (5 steps * 800ms + some margin)
  await page.waitForSelector('#scanner-results', { timeout: 10000 });
  await expect(page.locator('#scanner-results')).toBeVisible();
  await page.screenshot({ path: '/home/jules/verification/screenshots/exposure_tool_results.png' });

  console.log('Frontend verification for Exposure Tool complete.');
});

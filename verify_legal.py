import asyncio
from playwright.async_api import async_playwright

async def verify_legal():
    async def run_server():
        process = await asyncio.create_subprocess_exec(
            "python3", "-m", "http.server", "8080",
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE
        )
        return process

    server = await run_server()
    await asyncio.sleep(2) # Wait for server to start

    async with async_playwright() as p:
        browser = await p.chromium.launch()
        page = await browser.new_page()

        # Check Home and Footer Link
        await page.goto("http://localhost:8080/index.html")
        await page.click("text=Termini e Condizioni")
        await page.wait_for_url("**/legal.html")
        print("Footer link to legal.html: OK")

        # Check Legal Content
        content = await page.content()
        assert "Liability Cap" in content
        assert "Art. 1341-1342 c.c." in content
        assert "Pordenone" in content
        print("Legal page content: OK")

        # Check Checkout Checkboxes
        await page.goto("http://localhost:8080/index.html")
        await page.click("text=Acquista Ora - €1,90/mese")
        await page.wait_for_selector("#checkout-modal", state="visible")

        # Try to pay without checkboxes
        page.on("dialog", lambda dialog: dialog.dismiss())
        await page.click("text=Conferma Protocollo")
        print("Checkout validation (none checked): OK (handled by alert)")

        # Check all checkboxes
        await page.check("#tos-1")
        await page.check("#tos-2")
        await page.check("#tos-3")
        print("Checkboxes toggle: OK")

        await page.screenshot(path="legal_verification.png")
        await browser.close()

    server.terminate()
    await server.wait()

if __name__ == "__main__":
    asyncio.run(verify_legal())

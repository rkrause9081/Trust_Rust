// ====================== BID HELPERS ======================

function ethToWeiString(ethValue) {
    const raw = String(ethValue || "").trim();

    if (!raw || Number(raw) <= 0) {
        throw new Error("Amount must be greater than 0");
    }

    if (!/^\d+(\.\d{1,18})?$/.test(raw)) {
        throw new Error("Invalid ETH amount");
    }

    const [whole, fraction = ""] = raw.split(".");
    const fractionPadded = fraction.padEnd(18, "0");

    return (
        BigInt(whole || "0") * 10n ** 18n +
        BigInt(fractionPadded || "0")
    ).toString();
}

async function parseJsonResponse(res) {
    const text = await res.text();

    let data = {};
    try {
        data = text ? JSON.parse(text) : {};
    } catch {
        throw new Error(text || "Invalid server response");
    }

    if (!res.ok || data.success === false) {
        throw new Error(data.message || data.error || text || `Server error (${res.status})`);
    }

    return data;
}

// ====================== BID FORM HANDLING ======================

function setupBidForm() {
    const form = document.getElementById("bidForm");
    if (!form) return;

    form.addEventListener("submit", async (e) => {
        e.preventDefault();

        const statusEl = document.getElementById("bidStatus");
        const auctionAddress = document.getElementById("bidAuction")?.value;
        const bidAmountInput = document.getElementById("bidAmount")?.value;

        if (!auctionAddress || !bidAmountInput) {
            statusEl.textContent = "Please select an auction and enter a bid amount.";
            statusEl.style.color = "#ff4d6d";
            return;
        }

        let bidAmountWei;

        try {
            bidAmountWei = ethToWeiString(bidAmountInput);
        } catch (err) {
            statusEl.textContent = "❌ " + err.message;
            statusEl.style.color = "#ff4d6d";
            return;
        }

        statusEl.textContent = "Submitting bid on-chain...";
        statusEl.style.color = "#00e5ff";

        try {
            const res = await fetch("/api/bid", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
                body: JSON.stringify({
                    auction_address: auctionAddress,
                    bid_amount_wei: bidAmountWei,
                }),
            });

            const data = await parseJsonResponse(res);

            statusEl.innerHTML = `
                ✅ Bid placed successfully!<br>
                <strong>TX:</strong> ${data.tx_hash || "pending"}
            `;
            statusEl.style.color = "#32ff9a";

            form.reset();

            if (typeof loadActiveAuctions === "function") {
                await loadActiveAuctions();
            }
        } catch (err) {
            console.error("Bid submit error:", err);
            statusEl.textContent = "❌ " + err.message;
            statusEl.style.color = "#ff4d6d";
        }
    });
}

// ====================== WITHDRAW HANDLING ======================

async function handleWithdraw(auctionAddress) {
    try {
        const res = await fetch("/api/withdraw", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            credentials: "include",
            body: JSON.stringify({
                auction_address: auctionAddress,
            }),
        });

        const data = await parseJsonResponse(res);

        alert(`Withdraw successful!\nTX: ${data.tx_hash || "pending"}`);

        if (typeof loadActiveAuctions === "function") {
            await loadActiveAuctions();
        }
    } catch (err) {
        console.error("Withdraw error:", err);
        alert("Withdraw failed: " + err.message);
    }
}
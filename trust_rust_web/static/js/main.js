// ====================== CREATE AUCTION ======================
async function handleCreateAuction(e) {
    e.preventDefault();

    const statusEl = document.getElementById("createStatus");
    statusEl.textContent = "Creating auction on blockchain...";
    statusEl.style.color = "#00e5ff";

    const startingBidEth = parseFloat(document.getElementById("startingBid").value);
    const endDateStr = document.getElementById("endDate").value;

    if (!startingBidEth || !endDateStr) {
        statusEl.textContent = "❌ Please fill Starting Bid and End Date";
        statusEl.style.color = "#ff4d6d";
        return;
    }

    const startingBidWei = Math.floor(startingBidEth * 1e18).toString();
    const endTime = new Date(endDateStr).getTime();
    const now = Date.now();
    let biddingTimeSeconds = Math.floor((endTime - now) / 1000);
    if (biddingTimeSeconds < 60) biddingTimeSeconds = 86400;

    try {
        const res = await fetch("/api/create-auction", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            credentials: "include",
            body: JSON.stringify({
                bidding_time_seconds: biddingTimeSeconds,
                starting_bid_wei: startingBidWei
            })
        });

        const data = await res.json();

        if (data.success) {
            statusEl.innerHTML = `
                ✅ <strong>Auction created on-chain!</strong><br>
                <strong>Auction Address:</strong> ${data.auction_address}<br>
                <strong>Tx Hash:</strong> ${data.tx_hash}
            `;
            statusEl.style.color = "#32ff9a";
            e.target.reset();
            if (typeof loadActiveAuctions === 'function') loadActiveAuctions();
        } else {
            throw new Error(data.message || "Unknown error");
        }
    } catch (err) {
        console.error(err);
        statusEl.textContent = "❌ " + err.message;
        statusEl.style.color = "#ff4d6d";
    }
}

// ====================== LOGOUT ======================
function logout() {
    document.body.classList.remove("logged-in");
    const pill = document.getElementById("walletPill");
    if (pill) pill.style.display = "none";
    location.reload();
}

// ====================== INITIALIZE ======================
window.onload = function() {
    const createForm = document.getElementById("createAuctionForm");
    if (createForm) createForm.addEventListener("submit", handleCreateAuction);

    setupBidForm();

    window.siweLogin = siweLogin;
    window.logout = logout;
    window.loadActiveAuctions = loadActiveAuctions;
};
// ====================== BID FORM HANDLING ======================
function setupBidForm() {
    const form = document.getElementById('bidForm');
    if (!form) return;

    form.addEventListener('submit', async (e) => {
        e.preventDefault();

        const statusEl = document.getElementById('bidStatus');
        const auctionAddress = document.getElementById('bidAuction').value;
        const bidAmountEth = parseFloat(document.getElementById('bidAmount').value);

        if (!auctionAddress || !bidAmountEth) {
            statusEl.textContent = "Please select an auction and enter a bid amount";
            statusEl.style.color = "#ff4d6d";
            return;
        }

        const bidAmountWei = Math.floor(bidAmountEth * 1e18).toString();

        statusEl.textContent = "Submitting bid on-chain...";
        statusEl.style.color = "#00e5ff";

        try {
            const res = await fetch('/api/bid', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                credentials: 'include',
                body: JSON.stringify({
                    auction_address: auctionAddress,
                    bid_amount_wei: bidAmountWei
                })
            });

            console.log('Bid response status:', res.status);

            if (!res.ok) {
                const text = await res.text();
                console.error('Bid error response:', text);
                throw new Error(`Server error (${res.status})`);
            }

            const data = await res.json();
            console.log('Bid success response:', data);

            if (data.success) {
                statusEl.innerHTML = `✅ Bid placed successfully!<br>TX: ${data.tx_hash}`;
                statusEl.style.color = "#32ff9a";
                form.reset();
                if (typeof loadActiveAuctions === 'function') loadActiveAuctions();
            } else {
                throw new Error(data.message || "Bid failed");
            }
        } catch (err) {
            console.error('Bid submit error:', err);
            statusEl.textContent = "❌ " + err.message;
            statusEl.style.color = "#ff4d6d";
        }
    });
}
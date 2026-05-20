async function handleWithdraw(auctionAddress) {
    if (!confirm("Withdraw all pending funds from this auction?")) return;

    try {
        const res = await fetch('/api/withdraw', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            credentials: 'include',
            body: JSON.stringify({ auction_address: auctionAddress })
        });

        const data = await res.json();

        if (data.success) {
            alert(`✅ Withdrawal successful!\nAmount: ${data.amount_withdrawn_eth} ETH\nTx: ${data.tx_hash}`);
            if (typeof loadActiveAuctions === 'function') loadActiveAuctions();
        } else {
            alert("Withdrawal failed: " + (data.message || "Unknown error"));
        }
    } catch (err) {
        alert("Error: " + err.message);
    }
}
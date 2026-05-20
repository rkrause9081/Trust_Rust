// ====================== LOAD ACTIVE AUCTIONS ======================
async function loadActiveAuctions() {
    const grid = document.getElementById('auctionGrid');
    if (!grid) return;

    grid.innerHTML = '<p class="status-line">Loading auctions from blockchain...</p>';

    try {
        const res = await fetch('/api/auctions', { credentials: 'include' });

        if (!res.ok) {
            const text = await res.text();
            console.error(`[loadActiveAuctions] HTTP ${res.status}:`, text);
            throw new Error(`Server error (${res.status})`);
        }

        const data = await res.json();
        console.log('Loaded auctions:', data);

        let auctions = data.auctions || [];

        // Filter only active auctions
        const now = Math.floor(Date.now() / 1000);
        auctions = auctions.filter(a => parseInt(a.end_time) > now);

        populateBidDropdown(auctions);

        grid.innerHTML = '';

        if (auctions.length === 0) {
            grid.innerHTML = `
                <div class="card">
                    <p style="margin:0; color:#6b7c8f;">
                        No active auctions right now.<br>
                        <a href="#create-auction" style="color:#00e5ff;">Create the first one →</a>
                    </p>
                </div>
            `;
            return;
        }

        auctions.forEach(auction => {
            const card = document.createElement('div');
            card.className = 'card';
            card.style.cursor = 'pointer';

            const shortAuction = auction.auction_address.slice(0, 6) + '…' + auction.auction_address.slice(-4);
            const shortSeller  = auction.seller.slice(0, 6) + '…' + auction.seller.slice(-4);
            
            const startingEth = (parseFloat(auction.starting_bid_wei || "0") / 1e18).toFixed(4);
            const highestWei = auction.highest_bid_wei || auction.starting_bid_wei || "0";
            const highestEth = (parseFloat(highestWei) / 1e18).toFixed(4);

            const endTime = parseInt(auction.end_time);
            const secondsLeft = endTime - now;
            const days = Math.floor(secondsLeft / 86400);
            const hours = Math.floor((secondsLeft % 86400) / 3600);
            const timeLeftText = days > 0 ? `${days}d ${hours}h left` : `${hours}h left`;

            card.innerHTML = `
                <div class="card-title">Auction ${shortAuction}</div>
                <div style="margin: 0.75rem 0; font-size: 0.9rem; line-height: 1.5;">
                    <div><strong>Seller:</strong> ${shortSeller}</div>
                    <div><strong>Starting Bid:</strong> ${startingEth} ETH</div>
                    <div><strong>Highest Bid:</strong> <span style="color:#32ff9a; font-weight:600;">${highestEth} ETH</span></div>
                    <div><strong>Time Left:</strong> <span style="color:#32ff9a; font-weight:600;">${timeLeftText}</span></div>
                </div>
                <div style="display:flex; gap:0.5rem; margin-top:1rem;">
                    <button class="btn-primary" style="flex:1;">Place Bid →</button>
                    <button class="btn-secondary" style="flex:1;">Withdraw</button>
                </div>
            `;

            const placeBtn = card.querySelector('.btn-primary');
            const withdrawBtn = card.querySelector('.btn-secondary');

            placeBtn.onclick = (e) => {
                e.stopImmediatePropagation();
                const select = document.getElementById('bidAuction');
                if (select) select.value = auction.auction_address;
                document.getElementById('place-bid')?.scrollIntoView({ behavior: 'smooth' });
            };

            withdrawBtn.onclick = (e) => {
                e.stopImmediatePropagation();
                if (confirm(`Withdraw pending funds from auction ${shortAuction}?`)) {
                    handleWithdraw(auction.auction_address);
                }
            };

            card.onclick = () => placeBtn.click();

            grid.appendChild(card);
        });

    } catch (err) {
        console.error('loadActiveAuctions failed:', err);
        grid.innerHTML = `<div class="card"><p style="color:#ff4d6d; margin:0;">Error: ${err.message}</p></div>`;
    }
}
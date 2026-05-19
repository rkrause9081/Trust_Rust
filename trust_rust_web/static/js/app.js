/*
 * app.js
 *
 * Purpose:
 *     Frontend logic for SIWE authentication + real on-chain auction creation,
 *     loading active auctions, and placing bids.
 */

async function getNonce() {
    const res = await fetch("/auth/nonce");
    if (!res.ok) throw new Error("Failed to fetch nonce");
    const data = await res.json();
    return data.nonce;
}

function showWalletAddressModal(callback) {
    let modal = document.getElementById("walletModal");
    if (!modal) {
        modal = document.createElement("div");
        modal.id = "walletModal";
        modal.style.cssText = `
            position:fixed; top:0; left:0; right:0; bottom:0; background:rgba(8,11,15,0.95);
            display:flex; align-items:center; justify-content:center; z-index:9999;
            font-family:var(--sans);
        `;
        modal.innerHTML = `
            <div style="background:#0d1117; border:1px solid #00e5ff; padding:2rem; width:100%; max-width:420px; border-radius:8px; box-shadow:0 0 30px rgba(0,229,255,0.3);">
                <h2 style="color:#00e5ff; margin-bottom:1rem; font-size:1.4rem;">Enter Wallet Address</h2>
                <p style="color:#6b7c8f; font-size:0.9rem; margin-bottom:1.5rem;">
                    Paste any address from your Hardhat accounts
                </p>
                <input id="modalAddressInput" type="text" 
                       placeholder="0x39fd6e51aad88f6f4ce6ab882279cffb92266" 
                       style="width:100%; padding:1rem; background:#05070a; border:1px solid #1e2d40; color:#c9d6e3; font-family:var(--mono); font-size:1rem; margin-bottom:1.5rem; border-radius:4px;">
                <div style="display:flex; gap:1rem;">
                    <button onclick="hideWalletModal()" style="flex:1; padding:1rem; background:transparent; border:1px solid #1e2d40; color:#c9d6e3;">Cancel</button>
                    <button onclick="handleModalConfirm()" style="flex:1; padding:1rem; background:#00e5ff; color:#080b0f; font-weight:700;">Continue →</button>
                </div>
            </div>
        `;
        document.body.appendChild(modal);
    }
    modal.style.display = "flex";
    window.modalCallback = callback;
}

function hideWalletModal() {
    const modal = document.getElementById("walletModal");
    if (modal) modal.style.display = "none";
}

async function handleModalConfirm() {
    const input = document.getElementById("modalAddressInput");
    let address = input.value.trim().toLowerCase();

    if (!address || !address.startsWith("0x") || address.length !== 42) {
        alert("Please enter a valid Ethereum address (42 characters starting with 0x)");
        return;
    }

    hideWalletModal();

    try {
        const accounts = await ethereum.request({ method: "eth_requestAccounts" });

        if (!accounts.some(acc => acc.toLowerCase() === address)) {
            try {
                await ethereum.request({
                    method: "wallet_switchEthereumChain",
                    params: [{ chainId: "0x7a69" }]
                });
            } catch (e) {}
        }

        window.modalCallback(address);
    } catch (err) {
        alert("Failed to connect wallet: " + err.message);
    }
}

async function siweLogin() {
    if (!window.ethereum) {
        alert("MetaMask not found.");
        return;
    }

    showWalletAddressModal(async (userAddress) => {
        try {
            const nonce = await getNonce();
            const message = `${userAddress}\n\nSign in with Ethereum\n\nNonce: ${nonce}`;

            const signature = await ethereum.request({
                method: "personal_sign",
                params: [message, userAddress],
            });

            const res = await fetch("/auth/verify", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ message, signature }),
            });

            const data = await res.json();

            if (data.success) {
                console.log("✅ Logged in as:", data.wallet);
                document.body.classList.add("logged-in");

                const pill = document.getElementById("walletPill");
                pill.textContent = data.wallet.slice(0, 6) + "..." + data.wallet.slice(-4);
                pill.style.display = "inline-block";

                const heading = document.getElementById("walletHeading");
                if (heading) heading.textContent = data.wallet.slice(0, 6) + "..." + data.wallet.slice(-4);

                loadActiveAuctions();
            } else {
                alert("Login failed on backend");
            }
        } catch (err) {
            console.error(err);
            alert(err.code === 4001 ? "You cancelled the signature" : "Sign-in error: " + err.message);
        }
    });
}

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
            loadActiveAuctions();
        } else {
            throw new Error(data.message || "Unknown error");
        }
    } catch (err) {
        console.error(err);
        statusEl.textContent = "❌ " + err.message;
        statusEl.style.color = "#ff4d6d";
    }
}

// ====================== LOAD ACTIVE AUCTIONS ======================
async function loadActiveAuctions() {
    const grid = document.getElementById('auctionGrid');
    if (!grid) return;

    grid.innerHTML = '<p class="status-line">Loading auctions from blockchain...</p>';

    try {
        const res = await fetch('/api/auctions', { credentials: 'include' });
        if (!res.ok) throw new Error('Failed to fetch auctions from server');

        const data = await res.json();
        let auctions = data.auctions || [];

        // Filter only active auctions
        const now = Math.floor(Date.now() / 1000);
        auctions = auctions.filter(a => parseInt(a.end_time) > now);

        // Populate bid dropdown with current auctions
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
            const startingEth = (parseFloat(auction.starting_bid_wei) / 1e18).toFixed(4);

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
                    <div><strong>Time Left:</strong> <span style="color:#32ff9a; font-weight:600;">${timeLeftText}</span></div>
                </div>
                <button class="btn-primary" style="width:100%; margin-top:0.5rem;">
                    Place Bid →
                </button>
            `;

            const selectForBid = () => {
                const select = document.getElementById('bidAuction');
                if (select) select.value = auction.auction_address;
                document.getElementById('place-bid')?.scrollIntoView({ behavior: 'smooth' });
            };

            card.querySelector('button').onclick = (e) => {
                e.stopImmediatePropagation();
                selectForBid();
            };
            card.onclick = selectForBid;

            grid.appendChild(card);
        });

    } catch (err) {
        console.error(err);
        grid.innerHTML = `<div class="card"><p style="color:#ff4d6d; margin:0;">Error: ${err.message}</p></div>`;
    }
}

// Populate bid dropdown
function populateBidDropdown(auctions) {
    const select = document.getElementById('bidAuction');
    if (!select) return;

    select.innerHTML = '<option value="">-- Select an auction --</option>';

    auctions.forEach(auction => {
        const shortAddr = auction.auction_address.slice(0, 6) + '…' + auction.auction_address.slice(-4);
        const startingEth = (parseFloat(auction.starting_bid_wei) / 1e18).toFixed(4);

        const option = document.createElement('option');
        option.value = auction.auction_address;
        option.textContent = `${shortAddr} — Starting: ${startingEth} ETH`;
        select.appendChild(option);
    });
}

// ====================== BID FORM ======================
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

            const data = await res.json();

            if (data.success) {
                statusEl.innerHTML = `✅ Bid placed successfully!<br>TX: ${data.tx_hash}`;
                statusEl.style.color = "#32ff9a";
                form.reset();
                loadActiveAuctions(); // Refresh list after bidding
            } else {
                throw new Error(data.message || "Bid failed");
            }
        } catch (err) {
            statusEl.textContent = "❌ " + err.message;
            statusEl.style.color = "#ff4d6d";
        }
    });
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
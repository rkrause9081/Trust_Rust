// ====================== AUCTION HELPERS ======================

function formatAddress(address) {
    if (!address || typeof address !== "string") return "N/A";
    return address.slice(0, 6) + "…" + address.slice(-4);
}

function weiToEthString(wei) {
    try {
        const value = BigInt(wei || "0");
        const whole = value / 10n ** 18n;
        const fraction = value % 10n ** 18n;

        const fractionStr = fraction
            .toString()
            .padStart(18, "0")
            .slice(0, 4);

        return `${whole}.${fractionStr}`;
    } catch {
        return "0.0000";
    }
}

function getAuctionAddress(auction) {
    return auction.auction_address || auction.auctionAddress || "";
}

function getEndTime(auction) {
    return Number(auction.end_time || auction.endTime || 0);
}

function getStartingBidWei(auction) {
    return auction.starting_bid_wei || auction.startingBid || "0";
}

function getHighestBidWei(auction) {
    return auction.highest_bid_wei || auction.highestBid || "0";
}

function populateBidDropdown(auctions) {
    const select = document.getElementById("bidAuction");
    if (!select) return;

    select.innerHTML = `<option value="">Select an auction</option>`;

    auctions.forEach((auction) => {
        const address = getAuctionAddress(auction);
        const option = document.createElement("option");

        option.value = address;
        option.textContent = `${formatAddress(address)} — Highest: ${weiToEthString(getHighestBidWei(auction))} ETH`;

        select.appendChild(option);
    });
}

// ====================== LOAD ACTIVE AUCTIONS ======================

async function loadActiveAuctions() {
    const grid = document.getElementById("auctionGrid");
    if (!grid) return;

    grid.innerHTML = `<p class="status-line">Loading auctions from blockchain...</p>`;

    try {
        const res = await fetch("/api/auctions", {
            method: "GET",
            credentials: "include",
        });

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

        const now = Math.floor(Date.now() / 1000);

        let auctions = Array.isArray(data.auctions) ? data.auctions : [];

        auctions = auctions.filter((auction) => {
            const endTime = getEndTime(auction);
            return endTime > now;
        });

        populateBidDropdown(auctions);

        grid.innerHTML = "";

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

auctions.forEach((auction) => {
    const auctionAddress = getAuctionAddress(auction);

    const seller = auction.seller || "";
    const highestBidder = auction.highest_bidder || "";

    const title =
        auction.title ||
        `Auction ${formatAddress(auctionAddress)}`;

    const description =
        auction.description ||
        "No description provided.";

    const descriptionPreview =
        description.length > 120
            ? description.slice(0, 120) + "..."
            : description;

    const startingEth =
        weiToEthString(getStartingBidWei(auction));

    const highestWei =
        getHighestBidWei(auction);

    const highestEth =
        weiToEthString(
            highestWei === "0"
                ? getStartingBidWei(auction)
                : highestWei
        );

    const bidCount =
        Number(auction.bid_count || 0);

    const endTime = getEndTime(auction);

    const secondsLeft =
        Math.max(endTime - now, 0);

    const days =
        Math.floor(secondsLeft / 86400);

    const hours =
        Math.floor((secondsLeft % 86400) / 3600);

    const minutes =
        Math.floor((secondsLeft % 3600) / 60);

    let timeLeftText = `${minutes}m left`;

    if (days > 0) {
        timeLeftText = `${days}d ${hours}h left`;
    } else if (hours > 0) {
        timeLeftText = `${hours}h ${minutes}m left`;
    }

    const createdAt =
        Number(auction.created_at || 0);

    const createdAgo =
        Math.floor((now - createdAt) / 3600);

    const card = document.createElement("div");

    card.className = "card auction-card";

    card.innerHTML = `
        <div class="auction-image-placeholder">
            IMAGE SUPPORT COMING SOON
        </div>

        <div class="auction-title">
            ${title}
        </div>

        <div class="auction-address">
            ${auctionAddress}
        </div>

        <div class="auction-description">
            ${descriptionPreview}
        </div>

        <div class="auction-meta-grid">

            <div>
                <span class="meta-label">Highest Bid</span>
                <span class="meta-value accent">
                    ${highestEth} ETH
                </span>
            </div>

            <div>
                <span class="meta-label">Starting Bid</span>
                <span class="meta-value">
                    ${startingEth} ETH
                </span>
            </div>

            <div>
                <span class="meta-label">Bids</span>
                <span class="meta-value">
                    ${bidCount}
                </span>
            </div>

            <div>
                <span class="meta-label">Ends In</span>
                <span class="meta-value ${
                    secondsLeft < 3600
                        ? "danger"
                        : "success"
                }">
                    ${timeLeftText}
                </span>
            </div>

        </div>

        <div class="auction-footer">

            <div class="auction-seller">
                Seller:
                ${formatAddress(seller)}
            </div>

            <div class="auction-leading">
                Leading:
                ${
                    highestBidder ===
                    "0x0000000000000000000000000000000000000000"
                        ? "No bids yet"
                        : formatAddress(highestBidder)
                }
            </div>

            <div class="auction-created">
                Created ${createdAgo}h ago
            </div>

        </div>

        <div class="auction-actions">
            <button
                class="btn-primary place-bid-btn"
                type="button"
            >
                Place Bid →
            </button>

            <button
                class="btn-secondary withdraw-btn"
                type="button"
            >
                Withdraw
            </button>
        </div>
    `;

    const placeBtn =
        card.querySelector(".place-bid-btn");

    const withdrawBtn =
        card.querySelector(".withdraw-btn");

    placeBtn.onclick = (e) => {
        e.stopPropagation();

        const select =
            document.getElementById("bidAuction");

        if (select) {
            select.value = auctionAddress;
        }

        document
            .getElementById("place-bid")
            ?.scrollIntoView({
                behavior: "smooth",
            });
    };

    withdrawBtn.onclick = (e) => {
        e.stopPropagation();

        openWithdrawModal(auctionAddress);
    };

    grid.appendChild(card);
});
    } catch (err) {
        console.error("loadActiveAuctions failed:", err);

        grid.innerHTML = `
            <div class="card">
                <p style="color:#ff4d6d; margin:0;">Error: ${err.message}</p>
            </div>
        `;
    }
}
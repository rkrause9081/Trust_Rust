/*
 * auctions.js
 *
 * Purpose:
 *     Frontend auction rendering and blockchain auction
 *     display utilities.
 *
 * Responsibilities:
 *     - Fetch active auctions from the backend API
 *     - Render auction cards to the UI
 *     - Populate bid selection dropdowns
 *     - Format blockchain values for display
 *     - Display escrow status panels
 *     - Connect UI actions to auction interactions
 *
 * Non-Responsibilities:
 *     - Wallet authentication
 *     - Blockchain transaction signing
 *     - Backend API implementation
 *     - Persistent frontend state management
 *
 * Architecture:
 *
 *      Browser UI
 *           ↓
 *       auctions.js
 *           ↓
 *      REST API Routes
 *           ↓
 *      Blockchain Registry
 */

/* -------------------------------------------------------------------------- */
/*                              Auction Mappers                               */
/* -------------------------------------------------------------------------- */

/**
 * Retrieves the normalized auction contract address.
 *
 * Supports both snake_case and camelCase API formats.
 *
 * @param {Object} auction
 * @returns {string}
 */
function getAuctionAddress(auction) {
    return (
        auction.auction_address ||
        auction.auctionAddress ||
        ""
    );
}

/**
 * Retrieves the normalized auction end timestamp.
 *
 * @param {Object} auction
 * @returns {number}
 */
function getEndTime(auction) {
    return Number(
        auction.end_time ||
        auction.endTime ||
        0
    );
}

/**
 * Retrieves the normalized starting bid value.
 *
 * @param {Object} auction
 * @returns {string}
 */
function getStartingBidWei(auction) {
    return (
        auction.starting_bid_wei ||
        auction.startingBid ||
        "0"
    );
}

/**
 * Retrieves the normalized highest bid value.
 *
 * @param {Object} auction
 * @returns {string}
 */
function getHighestBidWei(auction) {
    return (
        auction.highest_bid_wei ||
        auction.highestBid ||
        "0"
    );
}

/**
 * Retrieves the normalized highest bidder address.
 *
 * @param {Object} auction
 * @returns {string}
 */
function getHighestBidder(auction) {
    return (
        auction.highest_bidder ||
        auction.highestBidder ||
        ""
    );
}

/* -------------------------------------------------------------------------- */
/*                           Bid Dropdown Rendering                           */
/* -------------------------------------------------------------------------- */

/**
 * Populates the auction bid selection dropdown.
 *
 * @param {Array<Object>} auctions
 */
function populateBidDropdown(auctions) {
    const select = document.getElementById("bidAuction");

    if (!select) {
        return;
    }

    select.innerHTML =
        `<option value="">Select an auction</option>`;

    auctions.forEach((auction) => {
        const address = getAuctionAddress(auction);

        const option = document.createElement("option");

        option.value = address;

        option.textContent =
            `${shortAddress(address, "…")} — Highest: ` +
            `${weiToEthString(getHighestBidWei(auction))} ETH`;

        select.appendChild(option);
    });
}

/* -------------------------------------------------------------------------- */
/*                           Auction Loading Logic                            */
/* -------------------------------------------------------------------------- */

/**
 * Loads auctions from the backend API and renders them to the UI.
 *
 * This function:
 *     - Fetches auction registry data
 *     - Filters active auctions
 *     - Renders auction cards
 *     - Loads escrow status information
 *     - Connects bid and withdrawal actions
 *
 * @returns {Promise<void>}
 */
async function loadActiveAuctions() {
    const grid = document.getElementById("auctionGrid");

    if (!grid) {
        return;
    }

    grid.innerHTML = `
        <p class="status-line">
            Loading auctions from blockchain...
        </p>
    `;

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
            throw new Error(
                text || "Invalid server response"
            );
        }

        if (!res.ok || data.success === false) {
            throw new Error(
                data.message ||
                data.error ||
                text ||
                `Server error (${res.status})`
            );
        }

        const now = Math.floor(Date.now() / 1000);

        let auctions = Array.isArray(data.auctions)
            ? data.auctions
            : [];

        auctions = auctions.filter((auction) => {
            return (
                getEndTime(auction) > now ||
                auction.exists
            );
        });

        populateBidDropdown(
            auctions.filter(
                (auction) => getEndTime(auction) > now
            )
        );

        grid.innerHTML = "";

        if (auctions.length === 0) {
            grid.innerHTML = `
                <div class="card">
                    <p style="margin:0; color:#6b7c8f;">
                        No auctions found.<br>
                        <a
                            href="#create-auction"
                            style="color:#00e5ff;"
                        >
                            Create the first one →
                        </a>
                    </p>
                </div>
            `;

            return;
        }

        for (const auction of auctions) {
            const auctionAddress =
                getAuctionAddress(auction);

            const seller = auction.seller || "";

            const highestBidder =
                getHighestBidder(auction);

            const title =
                auction.title ||
                `Auction ${shortAddress(auctionAddress, "…")}`;

            const description =
                auction.description ||
                "No description provided.";

            const descriptionPreview =
                description.length > 120
                    ? description.slice(0, 120) + "..."
                    : description;

            const startingEth = weiToEthString(
                getStartingBidWei(auction)
            );

            const highestWei =
                getHighestBidWei(auction);

            const highestEth =
                weiToEthString(highestWei);

            const bidCount = Number(
                auction.bid_count ||
                auction.bidCount ||
                0
            );

            const endTime = getEndTime(auction);

            const secondsLeft =
                Math.max(endTime - now, 0);

            const isActive = secondsLeft > 0;

            const days =
                Math.floor(secondsLeft / 86400);

            const hours = Math.floor(
                (secondsLeft % 86400) / 3600
            );

            const minutes = Math.floor(
                (secondsLeft % 3600) / 60
            );

            let timeLeftText = "Ended";

            if (isActive) {
                timeLeftText = `${minutes}m left`;

                if (days > 0) {
                    timeLeftText =
                        `${days}d ${hours}h left`;
                } else if (hours > 0) {
                    timeLeftText =
                        `${hours}h ${minutes}m left`;
                }
            }

            const createdAt = Number(
                auction.created_at ||
                auction.createdAt ||
                0
            );

            const createdAgoHours =
                createdAt > 0
                    ? Math.max(
                        Math.floor(
                            (now - createdAt) / 3600
                        ),
                        0
                    )
                    : null;

            let escrowPanel = "";

            if (
                typeof loadEscrowStatus === "function" &&
                typeof buildEscrowPanel === "function"
            ) {
                const escrow =
                    await loadEscrowStatus(
                        auctionAddress
                    );

                escrowPanel =
                    buildEscrowPanel(
                        auctionAddress,
                        escrow
                    );
            }

            const card =
                document.createElement("div");

            card.className =
                "card auction-card";

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
                        <span class="meta-label">
                            Highest Bid
                        </span>

                        <span class="meta-value accent">
                            ${highestEth} ETH
                        </span>
                    </div>

                    <div>
                        <span class="meta-label">
                            Starting Bid
                        </span>

                        <span class="meta-value">
                            ${startingEth} ETH
                        </span>
                    </div>

                    <div>
                        <span class="meta-label">
                            Bids
                        </span>

                        <span class="meta-value">
                            ${bidCount}
                        </span>
                    </div>

                    <div>
                        <span class="meta-label">
                            Status
                        </span>

                        <span class="meta-value ${
                            isActive
                                ? "success"
                                : "danger"
                        }">
                            ${timeLeftText}
                        </span>
                    </div>
                </div>

                <div class="auction-footer">
                    <div>
                        Seller:
                        ${shortAddress(seller, "…")}
                    </div>

                    <div>
                        Leading:

                        ${
                            !highestBidder ||
                            highestBidder ===
                            "0x0000000000000000000000000000000000000000"
                                ? "No bids yet"
                                : shortAddress(highestBidder, "…")
                        }
                    </div>

                    <div>
                        Created:

                        ${
                            createdAgoHours === null
                                ? "Unknown"
                                : `${createdAgoHours}h ago`
                        }
                    </div>
                </div>

                ${escrowPanel}

                <div class="auction-actions">
                    ${
                        isActive
                            ? `
                                <button
                                    class="btn-primary place-bid-btn"
                                    type="button"
                                >
                                    Place Bid →
                                </button>
                            `
                            : `
                                <button
                                    class="btn-ghost"
                                    type="button"
                                    disabled
                                >
                                    Auction Ended
                                </button>
                            `
                    }

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

            if (placeBtn) {
                placeBtn.onclick = (e) => {
                    e.stopPropagation();

                    const select =
                        document.getElementById(
                            "bidAuction"
                        );

                    if (select) {
                        select.value = auctionAddress;
                    }

                    document
                        .getElementById("place-bid")
                        ?.scrollIntoView({
                            behavior: "smooth",
                        });
                };
            }

            if (withdrawBtn) {
                withdrawBtn.onclick = (e) => {
                    e.stopPropagation();

                    if (
                        typeof openWithdrawModal !==
                        "function"
                    ) {
                        alert(
                            "Withdraw modal is not connected yet."
                        );

                        return;
                    }

                    openWithdrawModal(
                        auctionAddress
                    );
                };
            }

            grid.appendChild(card);
        }
    } catch (err) {
        console.error(
            "loadActiveAuctions failed:",
            err
        );

        grid.innerHTML = `
            <div class="card">
                <p style="color:#ff4d6d; margin:0;">
                    Error: ${err.message}
                </p>
            </div>
        `;
    }
}
/*
 * auction_cards.js
 *
 * Pure auction UI rendering.
 * No fetch calls belong here.
 */

function safeShortAddress(address, separator = "…") {
    if (typeof shortAddress === "function") {
        return shortAddress(address, separator);
    }

    if (!address || address.length < 12) {
        return address || "";
    }

    return `${address.slice(0, 6)}${separator}${address.slice(-4)}`;
}

function safeWeiToEthString(wei) {
    if (typeof weiToEthString === "function") {
        return weiToEthString(wei);
    }

    try {
        const value = BigInt(String(wei || "0"));
        const whole = value / 10n ** 18n;
        const fraction = (value % 10n ** 18n)
            .toString()
            .padStart(18, "0")
            .replace(/0+$/, "");

        return fraction ? `${whole}.${fraction}` : whole.toString();
    } catch {
        return "0";
    }
}

function isZeroAddress(address) {
    return !address ||
        address.toLowerCase() === "0x0000000000000000000000000000000000000000";
}

function getActiveAuctions(auctions) {
    const now = Math.floor(Date.now() / 1000);

    return auctions.filter((auction) => {
        return getEndTime(auction) > now;
    });
}

function renderDashboardStats(auctions) {
    const activeCount = document.getElementById("activeCount");
    const highestBid = document.getElementById("highestBid");
    const createdCount = document.getElementById("createdCount");

    const now = Math.floor(Date.now() / 1000);
    const activeAuctions = auctions.filter((auction) => getEndTime(auction) > now);

    const highestWei = auctions.reduce((max, auction) => {
        const current = BigInt(getHighestBidWei(auction) || "0");
        return current > max ? current : max;
    }, 0n);

    if (activeCount) {
        activeCount.textContent = String(activeAuctions.length);
    }

    if (highestBid) {
        highestBid.textContent = `${safeWeiToEthString(highestWei.toString())} ETH`;
    }

    if (createdCount) {
        createdCount.textContent = String(auctions.length);
    }
}

function populateBidDropdown(auctions) {
    const select = document.getElementById("bidAuction");

    if (!select) {
        return;
    }

    const selectedValue = select.value;

    select.innerHTML = `<option value="">Select an auction</option>`;

    getActiveAuctions(auctions).forEach((auction) => {
        const address = getAuctionAddress(auction);
        const option = document.createElement("option");

        option.value = address;
        option.textContent =
            `${safeShortAddress(address)} — Highest: ` +
            `${safeWeiToEthString(getHighestBidWei(auction))} ETH`;

        select.appendChild(option);
    });

    if (selectedValue) {
        select.value = selectedValue;
    }
}

function formatAuctionTimeLeft(endTime) {
    const now = Math.floor(Date.now() / 1000);
    const secondsLeft = Math.max(Number(endTime || 0) - now, 0);

    if (secondsLeft <= 0) {
        return {
            isActive: false,
            label: "Ended",
        };
    }

    const days = Math.floor(secondsLeft / 86400);
    const hours = Math.floor((secondsLeft % 86400) / 3600);
    const minutes = Math.floor((secondsLeft % 3600) / 60);

    if (days > 0) {
        return {
            isActive: true,
            label: `${days}d ${hours}h left`,
        };
    }

    if (hours > 0) {
        return {
            isActive: true,
            label: `${hours}h ${minutes}m left`,
        };
    }

    return {
        isActive: true,
        label: `${minutes}m left`,
    };
}

async function buildAuctionCard(auction) {
    const auctionAddress = getAuctionAddress(auction);
    const seller = auction.seller || "";
    const highestBidder = getHighestBidder(auction);
    const title = auction.title || `Auction ${safeShortAddress(auctionAddress)}`;
    const description = auction.description || "No description provided.";
    const descriptionPreview =
        description.length > 120 ? `${description.slice(0, 120)}...` : description;

    const startingEth = safeWeiToEthString(getStartingBidWei(auction));
    const highestEth = safeWeiToEthString(getHighestBidWei(auction));
    const bidCount = getBidCount(auction);
    const { isActive, label: timeLeftText } = formatAuctionTimeLeft(getEndTime(auction));

    const now = Math.floor(Date.now() / 1000);
    const createdAt = Number(auction.created_at || auction.createdAt || 0);
    const createdAgoHours =
        createdAt > 0 ? Math.max(Math.floor((now - createdAt) / 3600), 0) : null;

    let escrowPanel = "";

    if (typeof buildEscrowPanel === "function") {
        let escrow = null;

        if (typeof loadEscrowStatus === "function") {
            escrow = await loadEscrowStatus(auctionAddress);
        }

        escrowPanel = buildEscrowPanel(auctionAddress, escrow);
    }

    const card = document.createElement("div");
    card.className = "card auction-card";
    card.dataset.auctionAddress = auctionAddress;

    card.innerHTML = `
        <div class="auction-image-placeholder">
            IMAGE SUPPORT COMING SOON
        </div>

        <div class="auction-title">${title}</div>
        <div class="auction-address">${auctionAddress}</div>
        <div class="auction-description">${descriptionPreview}</div>

        <div class="auction-meta-grid">
            <div>
                <span class="meta-label">Highest Bid</span>
                <span class="meta-value accent">${highestEth} ETH</span>
            </div>

            <div>
                <span class="meta-label">Starting Bid</span>
                <span class="meta-value">${startingEth} ETH</span>
            </div>

            <div>
                <span class="meta-label">Bids</span>
                <span class="meta-value">${bidCount}</span>
            </div>

            <div>
                <span class="meta-label">Status</span>
                <span class="meta-value ${isActive ? "success" : "danger"}">
                    ${timeLeftText}
                </span>
            </div>
        </div>

        <div class="auction-footer">
            <div>Seller: ${safeShortAddress(seller)}</div>
            <div>
                Leading:
                ${
                    isZeroAddress(highestBidder)
                        ? "No bids yet"
                        : safeShortAddress(highestBidder)
                }
            </div>
            <div>
                Created:
                ${createdAgoHours === null ? "Unknown" : `${createdAgoHours}h ago`}
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

            <button class="btn-secondary withdraw-btn" type="button">
                Withdraw
            </button>
        </div>
    `;

    const placeBtn = card.querySelector(".place-bid-btn");
    const withdrawBtn = card.querySelector(".withdraw-btn");

    if (placeBtn) {
        placeBtn.addEventListener("click", (event) => {
            event.stopPropagation();

            const select = document.getElementById("bidAuction");

            if (select) {
                select.value = auctionAddress;
            }

            document.getElementById("place-bid")?.scrollIntoView({
                behavior: "smooth",
            });
        });
    }

    if (withdrawBtn) {
        withdrawBtn.addEventListener("click", (event) => {
            event.stopPropagation();

            if (typeof openWithdrawModal !== "function") {
                alert("Withdraw modal is not connected yet.");
                return;
            }

            openWithdrawModal(auctionAddress);
        });
    }

    return card;
}

async function renderAuctionGrid(auctions) {
    const grid = document.getElementById("auctionGrid");

    if (!grid) {
        return;
    }

    grid.innerHTML = "";

    if (!auctions.length) {
        grid.innerHTML = `
            <div class="card">
                <p style="margin:0; color:#6b7c8f;">
                    No auctions found.<br>
                    <a href="#create-auction" style="color:#00e5ff;">
                        Create the first one →
                    </a>
                </p>
            </div>
        `;
        return;
    }

    for (const auction of auctions) {
        const card = await buildAuctionCard(auction);
        grid.appendChild(card);
    }
}

async function renderAuctionUI(auctions) {
    const visibleAuctions = auctions.filter((auction) => auction.exists !== false);

    renderDashboardStats(visibleAuctions);
    populateBidDropdown(visibleAuctions);
    await renderAuctionGrid(visibleAuctions);
}

window.renderAuctionUI = renderAuctionUI;
window.populateBidDropdown = populateBidDropdown;

// ====================== MAIN HELPERS ======================

function ethToWeiString(ethValue) {
    const raw = String(ethValue || "").trim();

    if (raw === "") {
        throw new Error("Amount is required");
    }

    if (!/^\d+(\.\d{1,18})?$/.test(raw)) {
        throw new Error("Invalid ETH amount");
    }

    if (Number(raw) < 0) {
        throw new Error("Amount cannot be negative");
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
        throw new Error(
            data.message ||
            data.error ||
            text ||
            `Server error (${res.status})`
        );
    }

    return data;
}

// ====================== CREATE AUCTION ======================

async function handleCreateAuction(e) {
    e.preventDefault();

    const statusEl =
        document.getElementById("createStatus");

    const titleInput =
        document.getElementById("auctionTitle")?.value
            ?.trim();

    const descriptionInput =
        document.getElementById("auctionDescription")?.value
            ?.trim();

    const startingBidInput =
        document.getElementById("startingBid")?.value;

    const endDateStr =
        document.getElementById("endDate")?.value;

    statusEl.textContent =
        "Creating auction on blockchain...";

    statusEl.style.color = "#00e5ff";

    if (
        !titleInput ||
        !descriptionInput ||
        !endDateStr
    ) {
        statusEl.textContent =
            "❌ Please fill all required fields.";

        statusEl.style.color = "#ff4d6d";

        return;
    }

    let startingBidWei;
    let biddingTimeSeconds;

    try {
        startingBidWei =
            ethToWeiString(startingBidInput);

        const endTimeMs =
            new Date(endDateStr).getTime();

        const nowMs = Date.now();

        if (Number.isNaN(endTimeMs)) {
            throw new Error("Invalid end date.");
        }

        biddingTimeSeconds =
            Math.floor((endTimeMs - nowMs) / 1000);

        if (biddingTimeSeconds < 60) {
            throw new Error(
                "Auction end time must be at least 60 seconds in the future."
            );
        }
    } catch (err) {
        statusEl.textContent =
            "❌ " + err.message;

        statusEl.style.color = "#ff4d6d";

        return;
    }

    try {
        const res = await fetch(
            "/api/create-auction",
            {
                method: "POST",

                headers: {
                    "Content-Type":
                        "application/json",
                },

                credentials: "include",

                body: JSON.stringify({
                    bidding_time_seconds:
                        biddingTimeSeconds,

                    starting_bid_wei:
                        startingBidWei,

                    title: titleInput,

                    description:
                        descriptionInput,
                }),
            }
        );

        const data =
            await parseJsonResponse(res);

        statusEl.innerHTML = `
            ✅ <strong>Auction created on-chain!</strong><br>

            <strong>Auction Address:</strong><br>
            ${data.auction_address}<br><br>

            <strong>Tx Hash:</strong><br>
            ${data.tx_hash}
        `;

        statusEl.style.color = "#32ff9a";

        e.target.reset();

        if (
            typeof loadActiveAuctions ===
            "function"
        ) {
            await loadActiveAuctions();
        }
    } catch (err) {
        console.error(
            "Create auction error:",
            err
        );

        statusEl.textContent =
            "❌ " + err.message;

        statusEl.style.color = "#ff4d6d";
    }
}

// ====================== LOGOUT ======================

function logout() {
    document.body.classList.remove(
        "logged-in"
    );

    const pill =
        document.getElementById("walletPill");

    if (pill) {
        pill.textContent = "";
        pill.style.display = "none";
    }

    location.reload();
}

// ====================== INITIALIZE ======================

window.onload = function () {
    const createForm =
        document.getElementById(
            "createAuctionForm"
        );

    if (createForm) {
        createForm.addEventListener(
            "submit",
            handleCreateAuction
        );
    }

    if (
        typeof setupBidForm === "function"
    ) {
        setupBidForm();
    }

    window.siweLogin = siweLogin;
    window.logout = logout;
    window.loadActiveAuctions =
        loadActiveAuctions;

    if (
        typeof loadActiveAuctions ===
        "function"
    ) {
        loadActiveAuctions();
    }
    window.onload = function () {
    const createForm = document.getElementById("createAuctionForm");

    if (createForm) {
        createForm.addEventListener("submit", handleCreateAuction);
    }

    if (typeof setupBidForm === "function") {
        setupBidForm();
    }

    window.siweLogin = siweLogin;
    window.logout = logout;
    window.loadActiveAuctions = loadActiveAuctions;

    if (window.ethereum) {
        window.ethereum.on("accountsChanged", () => {
            alert("Wallet changed. Please sign in again.");
            logout();
        });
    }

    if (typeof loadActiveAuctions === "function") {
        loadActiveAuctions();
    }
};
};
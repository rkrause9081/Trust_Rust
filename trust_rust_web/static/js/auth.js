a// ====================== AUTH HELPERS ======================

function shortAddress(address) {
    if (!address || typeof address !== "string") return "";
    return address.slice(0, 6) + "..." + address.slice(-4);
}

async function getNonce() {
    const res = await fetch("/auth/nonce", {
        method: "GET",
        credentials: "include",
    });

    const text = await res.text();
    const data = text ? JSON.parse(text) : {};

    if (!res.ok) {
        throw new Error(data.message || data.error || text || "Failed to fetch nonce");
    }

    return data.nonce;
}

function setLoggedInWallet(wallet) {
    document.body.classList.add("logged-in");

    const pill = document.getElementById("walletPill");
    if (pill) {
        pill.textContent = shortAddress(wallet);
        pill.style.display = "inline-block";
    }

    const heading = document.getElementById("walletHeading");
    if (heading) {
        heading.textContent = shortAddress(wallet);
    }
}

// ====================== WALLET MODAL ======================

function showWalletAddressModal(callback) {
    let modal = document.getElementById("walletModal");

    if (!modal) {
        modal = document.createElement("div");
        modal.id = "walletModal";
        modal.style.cssText = `
            position:fixed;
            top:0;
            left:0;
            right:0;
            bottom:0;
            background:rgba(8,11,15,0.95);
            display:flex;
            align-items:center;
            justify-content:center;
            z-index:9999;
            font-family:var(--sans);
        `;

        modal.innerHTML = `
            <div style="background:#0d1117; border:1px solid #00e5ff; padding:2rem; width:100%; max-width:420px; border-radius:8px; box-shadow:0 0 30px rgba(0,229,255,0.3);">
                <h2 style="color:#00e5ff; margin-bottom:1rem; font-size:1.4rem;">Enter Wallet Address</h2>

                <p style="color:#6b7c8f; font-size:0.9rem; margin-bottom:1.5rem;">
                    Paste one of your Hardhat account addresses.
                </p>

                <input
                    id="modalAddressInput"
                    type="text"
                    placeholder="0x..."
                    style="width:100%; padding:1rem; background:#05070a; border:1px solid #1e2d40; color:#c9d6e3; font-family:var(--mono); font-size:1rem; margin-bottom:1.5rem; border-radius:4px;"
                >

                <div style="display:flex; gap:1rem;">
                    <button
                        type="button"
                        onclick="hideWalletModal()"
                        style="flex:1; padding:1rem; background:transparent; border:1px solid #1e2d40; color:#c9d6e3;"
                    >
                        Cancel
                    </button>

                    <button
                        type="button"
                        onclick="handleModalConfirm()"
                        style="flex:1; padding:1rem; background:#00e5ff; color:#080b0f; font-weight:700;"
                    >
                        Continue →
                    </button>
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
    const address = input.value.trim();

    if (!/^0x[a-fA-F0-9]{40}$/.test(address)) {
        alert("Please enter a valid Ethereum address.");
        return;
    }

    hideWalletModal();

    try {
        await ethereum.request({
            method: "eth_requestAccounts",
        });

        if (typeof window.modalCallback === "function") {
            await window.modalCallback(address);
        }
    } catch (err) {
        alert("Failed to connect wallet: " + err.message);
    }
}

// ====================== SIWE LOGIN ======================

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
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
                body: JSON.stringify({
                    message,
                    signature,
                }),
            });

            const text = await res.text();
            const data = text ? JSON.parse(text) : {};

            if (!res.ok || !data.success) {
                throw new Error(data.message || data.error || text || "Login failed on backend");
            }

            console.log("Logged in as:", data.wallet);

            setLoggedInWallet(data.wallet);

            if (typeof loadActiveAuctions === "function") {
                await loadActiveAuctions();
            }
        } catch (err) {
            console.error(err);

            if (err.code === 4001) {
                alert("You cancelled the signature.");
            } else {
                alert("Sign-in error: " + err.message);
            }
        }
    });
}
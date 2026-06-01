/*
 * auth_session.js
 *
 * SIWE login and session UI.
 */

async function getNonce() {
    const res = await fetch("/auth/nonce", {
        method: "GET",
        credentials: "include",
    });

    return window.auctionApi
        ? apiParseJsonResponse(res).then((data) => data.nonce)
        : parseJsonResponse(res).then((data) => data.nonce);
}

function setLoggedInWallet(wallet) {
    document.body.classList.add("logged-in");
    localStorage.setItem("trust_wallet", wallet);

    const loginBtn = document.getElementById("loginBtn");
    const pill = document.getElementById("walletPill");
    const heading = document.getElementById("walletHeading");

    if (loginBtn) {
        loginBtn.style.display = "none";
    }

    if (pill) {
        pill.textContent = safeShortAddress(wallet);
        pill.style.display = "inline-block";
        pill.style.cursor = "pointer";
        pill.title = "Click to logout";

        pill.onclick = async () => {
            const confirmed = confirm("Logout and clear session?");

            if (confirmed) {
                await logout();
            }
        };
    }

    if (heading) {
        heading.textContent = safeShortAddress(wallet);
    }
}

function restoreWalletUiFromStorage() {
    const wallet = localStorage.getItem("trust_wallet");

    if (wallet) {
        setLoggedInWallet(wallet);
    }
}

function hideWalletModal() {
    document.getElementById("walletModal")?.remove();
}

function showWalletAddressModal(callback) {
    let modal = document.getElementById("walletModal");

    if (!modal) {
        modal = document.createElement("div");
        modal.id = "walletModal";
        modal.style.cssText = `
            position:fixed;
            inset:0;
            background:rgba(8,11,15,0.95);
            display:flex;
            align-items:center;
            justify-content:center;
            z-index:9999;
            font-family:var(--sans);
        `;

        modal.innerHTML = `
            <div style="background:#0d1117; border:1px solid #00e5ff; padding:2rem; width:100%; max-width:420px; border-radius:8px; box-shadow:0 0 30px rgba(0,229,255,0.3);">
                <h2 style="color:#00e5ff; margin-bottom:1rem; font-size:1.4rem;">
                    Enter Wallet Address
                </h2>

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
                        id="walletModalCancel"
                        style="flex:1; padding:1rem; background:transparent; border:1px solid #1e2d40; color:#c9d6e3;"
                    >
                        Cancel
                    </button>

                    <button
                        type="button"
                        id="walletModalContinue"
                        style="flex:1; padding:1rem; background:#00e5ff; border:1px solid #00e5ff; color:#05070a;"
                    >
                        Continue →
                    </button>
                </div>
            </div>
        `;

        document.body.appendChild(modal);
    }

    const input = document.getElementById("modalAddressInput");
    const cancel = document.getElementById("walletModalCancel");
    const submit = document.getElementById("walletModalContinue");

    if (input) {
        input.value = "";
        input.focus();
    }

    if (cancel) {
        cancel.onclick = hideWalletModal;
    }

    if (submit) {
        submit.onclick = () => {
            const address = input?.value.trim();

            if (!address || !/^0x[a-fA-F0-9]{40}$/.test(address)) {
                alert("Please enter a valid Ethereum address.");
                return;
            }

            hideWalletModal();
            callback(address);
        };
    }
}

async function signWithWallet(message, walletAddress) {
    if (!window.ethereum) {
        throw new Error("MetaMask is not available.");
    }

    const accounts = await window.ethereum.request({
        method: "eth_requestAccounts",
    });

    const activeAccount = accounts[0];

    if (
        walletAddress &&
        activeAccount &&
        walletAddress.toLowerCase() !== activeAccount.toLowerCase()
    ) {
        console.warn(
            "Typed wallet differs from MetaMask selected account.",
            { typed: walletAddress, selected: activeAccount }
        );
    }

    return await window.ethereum.request({
        method: "personal_sign",
        params: [message, activeAccount],
    });
}

async function siweLogin() {
    showWalletAddressModal(async (walletAddress) => {
        try {
            const nonce = await getNonce();

            const message = [
                walletAddress,
                "",
                "Sign in with Ethereum",
                "",
                `Nonce: ${nonce}`,
            ].join("\n");

            const signature = await signWithWallet(message, walletAddress);

            const res = await fetch("/auth/verify", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                credentials: "include",
                body: JSON.stringify({ message, signature }),
            });

            const data = await apiParseJsonResponse(res);

            setLoggedInWallet(data.wallet);

            if (typeof refreshAuctions === "function") {
                await refreshAuctions({ showLoading: true });
            }
        } catch (err) {
            console.error("SIWE login failed:", err);

            if (err.code === 4001) {
                alert("You cancelled the signature.");
                return;
            }

            alert(`Sign-in error: ${err.message}`);
        }
    });
}

/*
 * Intentional no-op: logout button is being removed from the UI.
 * Kept only so old HTML does not throw if the button still exists while replacing files.
 */
async function logout() {
    try {
        await fetch("/auth/logout", {
            method: "POST",
            credentials: "include",
        });
    } catch (err) {
        /*
         * Still clear the browser-side wallet state even if the server
         * is already down or the session cookie is already expired.
         */
        console.error("Server logout failed:", err);
    } finally {
        localStorage.removeItem("trust_wallet");
        document.body.classList.remove("logged-in");
        location.reload();
    }
}

window.siweLogin = siweLogin;
window.logout = logout;
window.hideWalletModal = hideWalletModal;
window.restoreWalletUiFromStorage = restoreWalletUiFromStorage;
window.setLoggedInWallet = setLoggedInWallet;

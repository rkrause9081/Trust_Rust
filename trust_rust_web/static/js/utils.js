/*
 * utils.js
 *
 * Purpose:
 *     Shared frontend utility helpers used across the
 *     T.R.U.S.T Auction Protocol UI.
 *
 * Responsibilities:
 *     - Format Ethereum addresses for display
 *     - Parse backend API responses consistently
 *
 * Non-Responsibilities:
 *     - Wallet authentication
 *     - Auction rendering
 *     - Blockchain transaction execution
 *     - DOM initialization
 *
 * Architecture:
 *
 *      Frontend Modules
 *             ↓
 *           utils.js
 *             ↓
 *      Shared Helper Logic
 */

/* -------------------------------------------------------------------------- */
/*                           Ethereum Formatting                              */
/* -------------------------------------------------------------------------- */

/**
 * Formats an Ethereum address for compact UI display.
 *
 * Example:
 *     0x1234...abcd
 *
 * @param {string} address
 * @param {string} [separator="..."]
 * @returns {string}
 */
function shortAddress(address, separator = "...") {
    if (!address || typeof address !== "string") {
        return "";
    }

    return (
        address.slice(0, 6) +
        separator +
        address.slice(-4)
    );
}

/* -------------------------------------------------------------------------- */
/*                              API Utilities                                 */
/* -------------------------------------------------------------------------- */

/**
 * Parses a backend response into JSON and throws consistent API errors.
 *
 * @param {Response} res
 * @returns {Promise<Object>}
 * @throws {Error} If the response is invalid or unsuccessful.
 */
async function parseJsonResponse(res) {
    const text = await res.text();

    let data = {};

    try {
        data = text
            ? JSON.parse(text)
            : {};
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

    return data;
}
function weiToEthString(wei) {
  if (wei === undefined || wei === null) return "0";

  try {
    return ethers.formatEther(wei.toString());
  } catch {
    return "0";
  }
}

window.weiToEthString = weiToEthString;
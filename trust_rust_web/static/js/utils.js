function populateBidDropdown(auctions) {
    const select = document.getElementById('bidAuction');
    if (!select) return;

    select.innerHTML = '<option value="">-- Select an auction --</option>';

    auctions.forEach(auction => {
        const shortAddr = auction.auction_address.slice(0, 6) + '…' + auction.auction_address.slice(-4);
        const startingEth = (parseFloat(auction.starting_bid_wei || "0") / 1e18).toFixed(4);

        const option = document.createElement('option');
        option.value = auction.auction_address;
        option.textContent = `${shortAddr} — Starting: ${startingEth} ETH`;
        select.appendChild(option);
    });
}
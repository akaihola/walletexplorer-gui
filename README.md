WalletExplorer GUI
==================

This is a network graph visualization tool for the Bitcoin blockchain. It is a GUI for the [WalletExplorer.com][w] API.


Quickstart
----------

Pre-requisites:

- [Rust][r]
- [Cargo][c]
- a web browser

1. Clone the repository

    ```bash
    git clone https://github.com/akaihola/walletexplorer-gui.git
    cd walletexplorer-gui
    ```
   
2. Run the GUI

    ```bash
    cargo run
    ```
   
3. Open the GUI in a web browser

    Open the URL `http://localhost:3030` in a web browser.

4. Enter a Bitcoin transaction ID in the input field and click the "Add" button.

   The GUI will fetch the transaction data from the WalletExplorer API and display it in the network graph.

5. Select an address node and click the "Expand selected address" button.

   The GUI will fetch the next transaction involving the selected address and add it to the network graph.

Other things you can do:
- Hover over a transaction, address or arrow to see more information.
- Click and drag a node to move it around horizontally.
- Use the mouse wheel or the Page Up / Page Down keys to zoom in and out.
- Drag on the background or use the arrow keys to pan the view.
- Click on the "Download" button to save the network graph as a PNG image.
- Reload the page to start over.

License
-------

`walletexplorer-gui` is dual licensed under both

  1. The Apache 2.0 License
     http://www.apache.org/licenses/LICENSE-2.0

     and

  2. The MIT License
     http://opensource.org/licenses/MIT

`walletexplorer-gui` may be distributed under either license.

[w]: https://www.walletexplorer.com/
[r]: https://www.rust-lang.org/tools/install
[c]: https://doc.rust-lang.org/cargo/getting-started/installation.html

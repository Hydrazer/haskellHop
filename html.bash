'/mnt/d/Program Files/PowerShell/7/pwsh.exe' 'D:\\Program Files\\PowerShell\\7\\bevy_wasm.ps1'
# cargo build --release --target wasm32-unknown-unknown
# wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/haskell_hop.wasm

s=$(echo '
const observer = new MutationObserver((mutations, obs) => {
  var canvas = document.getElementsByTagName("canvas")[0];
  if (canvas) {
    console.log("nice");
    canvas.focus();
    obs.disconnect();
    return;
  }
});

observer.observe(document, {
  childList: true,
  subtree: true
});
' | cat - 'out/haskell_hop.js')


echo "$s" > 'out/haskell_hop.js'


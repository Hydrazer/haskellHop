'/mnt/d/Program Files/PowerShell/7/pwsh.exe' 'D:\\Program Files\\PowerShell\\7\\bevy_wasm.ps1'
mv out/haskell_hop.js out/bruh.js
echo '
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
' | cat - 'out/bruh.js' > 'out/haskell_hop.js'

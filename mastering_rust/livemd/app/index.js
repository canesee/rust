import('./livemd').then((livemd) => {
    var editor = document.getElementById("editor");
    var preview = document.getElementById("preview");
    var markdownToHtml = function() {
    var markdownText = editor.value;
    html = livemd.parse(markdownText);
    preview.innerHTML = html;
    };
    editor.addEventListener('input', markdownToHtml);
    // 开始解析初始文本
    markdownToHtml();
   }).catch(console.error); 
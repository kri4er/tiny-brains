## Enable Diagrams in our blog

### Prelude
 I'm bad at technical writing, I'm also bat at writing in general. The reason I started this blog is to improve my writing skills. 
 Most of my life I'm writing either embedded either parts of software, like Optimized text rendering using Nvidia Cuda, processing overlapping ranges of 
 price rows. Moving JSON/Paruqet around. Improving ETL performance. Building micro-services to index large amount of data.

While working on these problems I used visualization tools, like draw.io or some internal ones, or even dashboard. Having that said, 
I never had a good diagram from the first time. Step by step evolving it together with text, and the idea in general. 
I love and respect iterations.

### So what requirements this approach puts on the tooling
Following the iterations approach, I need a tool that can edit text and diagrams, they do not have to be the same tool, but they must be in sync.
When I'm updating the diagram and commit my changes, the document should also get updated with new version of diagram.

### Requirements
 1. Diagram versioning
 2. Diagrams are rendered on my document/blog
 3. Diagram is easy to edit/update using well known tool
 4. No complicated stack build to adopt draw.io format
 5. No SVG/PNG conversion anywhere except client side

 Fulfilling this list is not such a hard deal, right?! RIGHT?!!
 Well, lets do it step by step:

 1. Versioning - can be solved by git repo.
 2. Rendering - just make sure diagram in format html can render
 3. Tool must be able to edit and save the diagram

 I have searched the interned, and looks like draw.io can do that all, and Mr `markNZed` have it all figured out already.
 Awesome, lets just follow steps of this great person. [His post](https://github.com/jgraph/drawio/discussions/3430#discussioncomment-5322399)

What we need to have at the end

### Implementation
Import script form draw.io on our blog header:
```html
<html>
<head>
    <script type="text/javascript" src="https://www.draw.io/js/viewer.min.js"></script>
</head>
```
Great, but that is not enough, we now need to fetch our draw.io file from somewhere.
I'll put it into my repository and use code from the comment, here is how it looks like:
```js
        <script>
            (function () {
                // Escape HTML
                const chatMap = {
                    "&": "&amp;",
                    "'": "&#x27;",
                    "`": "&#x60;",
                    '"': "&quot;",
                    "<": "&lt;",
                    ">": "&gt;",
                };

                function replaceMatchedCharacters(match) {
                    return chatMap[match];
                }

                function escapeHTML(string) {
                    if (typeof string !== "string") return string;
                    return string.replace(/[&'`"<>]/g, replaceMatchedCharacters);
                }

                // Draw.io converter
                function createMxGraphData(xml, idx = new Date().getTime()) {
                    return {
                        editable: false,
                        highlight: "#0000ff",
                        nav: false,
                        toolbar: null,
                        edit: null,
                        resize: true,
                        lightbox: "open",
                        xml,
                    };
                }

                async function drawioConverterAsync(xml, idx) {
                    return new Promise((resolve) => {
                        const mxGraphData = createMxGraphData(xml, idx);
                        const json = JSON.stringify(mxGraphData);
                        const mxGraphHTML = createMxGraphHTML(json);
                        resolve(mxGraphHTML);
                    });
                }

                function createMxGraphHTML(json) {
                    return `<div class="mxgraph" style="max-width:100%;border:1px solid transparent;" data-mxgraph="${escapeHTML(json)}"></div>`;
                }

                // Load Draw.io file
                function loadDrawioFile(url) {
                    fetch(url)
                        .then((response) => {
                            return response.text();
                        })
                        .then((data) => {
                            return drawioConverterAsync(data);
                        })
                        .then((content) => {
                            const graphContainer = document.getElementById('drawio-diagram');
                            graphContainer.innerHTML = content;
                            window.GraphViewer.processElements();
                        })
                        .catch((err) => {
                            console.error("Error loading draw.io file:", err);
                        });
                }

                // Initialize
                loadDrawioFile("https://raw.githubusercontent.com/kri4er/tiny-brains/main/assets/Untitled%20Diagram.drawio");
            })();
        </script>
```
This scrip will do a bit of magic, but there are two important places here:
Here we call our script function to load our github hosted drawio file. I do not know what throttling limits are there, but I hope it won't be an issue.
```js
                loadDrawioFile("https://raw.githubusercontent.com/kri4er/tiny-brains/main/assets/Untitled%20Diagram.drawio");
```
and here we tell what container of html will be used to place the diagram by id:
```js
                            const graphContainer = document.getElementById('drawio-diagram');
```
Since I'm using generated markdown, that supports custom ids for header components, we can utilize it as a way to represent diagram position]
in the document, like this:
```
# Diagram { #drawio-diagram }
```
It will generate next html code:
```html
<h1> id="drawio-diagram"></h1>
```
And the viewer.js will make sure to convert drawio file to svg, nice and strong.


# Diagram { #drawio-diagram }


This is good enough for us, for now. Next time we will improve the script to automatically inserting all drawIo links into the document.
Feel free to inspect this html to see what is happening here, or check the code on my [github page](https://github.com/kri4er/tiny-brains)

Thank you for reading.

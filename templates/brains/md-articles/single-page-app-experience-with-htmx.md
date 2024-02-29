## Single page application(SPA) experience on example of pagination via HTMX + Askama for template fragments

### Prelude
Right now, every page is a separate template, rendered on request.
Thought it works great for our regular pages, 
it is quite annoying that I have to adjust html every-time I plan to add new post.

### Template fragments requirements
 1. When merged - with other fragment - provides valid html
 2. Each fragment can be tested on it's own
 3. Template is strongly typed and connects to data used to create a fragment
 4. Fragment can be used as standalone entity on html page

### Approach
We know that askama allows to inherit/compose other templates. Great that must be enough
to support fragments. Askama is type safe template engine. The composition/inheritance
solve the need to use fragment as standalone entity. And lastly htmx will give us power to 
load that entity into our responsive SPA.

### Execution
Comment integration would require some kind of database to store entities. That would be in future.
Lets start simple:
An article with multiple stories inside. It should have some static content, like description.
Then a list of available sub-stories to select. And when one selected, it is pulled from our back-end
and rendered. This is the most simple pagination possible. Where pages are fragments of our article.


TODO.

Enough for now, next part will include :
1. Example of big html, and it being split on fragments using askama.
2. Using htmx to set up event based update to the content
3. Encode state into url, so we do not have to keep it

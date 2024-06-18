// literally just a server that serves the files in the dist folder
import { serve, file } from 'bun';
serve({
    port: 443,
    async fetch(req) {
        const path = new URL(req.url).pathname;
        let filename = path === "/" ? "index.html" : path.slice(1);
        let result = file('dist/' + filename)
        if (!await result.exists()) return new Response('not found', { status: 404 });
        return new Response(result);
    },
    // WHATEVER YOU DO DON'T COMMIT THE PRIVATE KEY
    tls: {
        cert: Bun.file("../ssl/certificate.crt"),
        key: Bun.file("../ssl/private.key"),
    },
});


serve({
    port: 80,
    fetch(req) {
        const url = new URL(req.url);
        return Response.redirect('https://' + url.host + url.pathname)
    }
})
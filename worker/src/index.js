const CACHE_TTL_SECONDS = 3600; // 1 ora

export default {
  async fetch(request, env, ctx) {
    const allowedOrigin = env.ALLOWED_ORIGIN;

    if (request.method === "OPTIONS") {
      return new Response(null, {
        headers: corsHeaders(allowedOrigin),
      });
    }

    const url = new URL(request.url);
    const courseId = url.searchParams.get("course");
    const start = url.searchParams.get("start");
    const end = url.searchParams.get("end");

    if (!courseId || !start || !end) {
      return jsonError("Missing params", 400, allowedOrigin);
    }

    if (!/^\d+$/.test(courseId)) {
      return jsonError("Invalid course id", 400, allowedOrigin);
    }

    const dateRegex = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$/;
    if (!dateRegex.test(start) || !dateRegex.test(end)) {
      return jsonError("Invalid date format", 400, allowedOrigin);
    }

    const targetUrl = `https://corsidilaurea.uniroma1.it/it/services/gomp/timetable-data/${courseId}?start=${start}&end=${end}`;

    const cache = caches.default;
    const cacheKey = new Request(targetUrl, request);
    let response = await cache.match(cacheKey);

    if (!response) {
      const upstream = await fetch(targetUrl);
      const body = await upstream.text();

      response = new Response(body, {
        status: upstream.status,
        headers: {
          "Content-Type": "application/json",
          "Cache-Control": `public, max-age=${CACHE_TTL_SECONDS}`,
        },
      });

      if (upstream.ok) {
        ctx.waitUntil(cache.put(cacheKey, response.clone()));
      }
    }

    const finalHeaders = new Headers(response.headers);
    for (const [key, value] of Object.entries(corsHeaders(allowedOrigin))) {
      finalHeaders.set(key, value);
    }

    return new Response(response.body, {
      status: response.status,
      headers: finalHeaders,
    });
  },
};

function corsHeaders(allowedOrigin) {
  return {
    "Access-Control-Allow-Origin": allowedOrigin,
    "Access-Control-Allow-Methods": "GET, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type",
  };
}

function jsonError(message, status, allowedOrigin) {
  return new Response(JSON.stringify({ error: message }), {
    status,
    headers: {
      "Content-Type": "application/json",
      ...corsHeaders(allowedOrigin),
    },
  });
}
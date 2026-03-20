# API Routes

API routes in NestForge Web handle HTTP requests and return JSON. They live in the `src/app/api/` directory.

## Basic Route

Create a route file at `src/app/api/hello/route.ts`:

```typescript
export async function GET() {
  return Response.json({ message: "Hello, World!" });
}
```

Access at: `GET /api/hello`

## HTTP Methods

Export named functions for each HTTP method:

```typescript
// src/app/api/users/route.ts

export async function GET(request: Request) {
  const users = await getUsers();
  return Response.json(users);
}

export async function POST(request: Request) {
  const body = await request.json();
  const user = await createUser(body);
  return Response.json(user, { status: 201 });
}

export async function PUT(request: Request) {
  const body = await request.json();
  const user = await updateUser(body);
  return Response.json(user);
}

export async function DELETE(request: Request) {
  await deleteUser();
  return new Response(null, { status: 204 });
}
```

## Request Handling

### Reading Query Parameters

```typescript
export async function GET(request: Request) {
  const url = new URL(request.url);
  const page = url.searchParams.get("page") ?? "1";
  const limit = url.searchParams.get("limit") ?? "10";

  const users = await getUsers({ page, limit });
  return Response.json(users);
}
```

### Reading Headers

```typescript
export async function GET(request: Request) {
  const authHeader = request.headers.get("Authorization");
  const contentType = request.headers.get("Content-Type");

  if (!authHeader) {
    return Response.json({ error: "Unauthorized" }, { status: 401 });
  }

  return Response.json({ authenticated: true });
}
```

### Reading the Request Body

```typescript
export async function POST(request: Request) {
  try {
    const body = await request.json();
    const user = await createUser(body);
    return Response.json(user, { status: 201 });
  } catch (error) {
    return Response.json(
      { error: "Invalid JSON body" },
      { status: 400 }
    );
  }
}
```

### Form Data

```typescript
export async function POST(request: Request) {
  const formData = await request.formData();
  const name = formData.get("name") as string;
  const email = formData.get("email") as string;

  return Response.json({ name, email });
}
```

## Response Helpers

### JSON Response

```typescript
return Response.json(data);
return Response.json(data, { status: 201 });
```

### Empty Response

```typescript
return new Response(null, { status: 204 });
```

### Streaming Response

```typescript
export async function GET() {
  const stream = new ReadableStream({
    start(controller) {
      controller.enqueue("Hello\n");
      controller.enqueue("World\n");
      controller.close();
    },
  });

  return new Response(stream);
}
```

## Error Handling

### Basic Error Response

```typescript
export async function GET(request: Request) {
  const url = new URL(request.url);
  const id = url.searchParams.get("id");

  if (!id) {
    return Response.json(
      { error: "Missing 'id' parameter" },
      { status: 400 }
    );
  }

  const user = await getUserById(id);

  if (!user) {
    return Response.json(
      { error: "User not found" },
      { status: 404 }
    );
  }

  return Response.json(user);
}
```

### Try/Catch Pattern

```typescript
export async function GET(request: Request) {
  try {
    const user = await database.users.findMany();
    return Response.json(user);
  } catch (error) {
    console.error("Database error:", error);
    return Response.json(
      { error: "Internal server error" },
      { status: 500 }
    );
  }
}
```

## Dynamic Routes

### With Parameters

```typescript
// src/app/api/users/[id]/route.ts
export async function GET(
  request: Request,
  { params }: { params: { id: string } }
) {
  const user = await getUserById(params.id);

  if (!user) {
    return Response.json({ error: "User not found" }, { status: 404 });
  }

  return Response.json(user);
}
```

### Multiple Parameters

```typescript
// src/app/api/posts/[year]/[month]/[day]/route.ts
export async function GET(
  request: Request,
  { params }: { params: { year: string; month: string; day: string } }
) {
  const date = `${params.year}-${params.month}-${params.day}`;
  const posts = await getPostsByDate(date);
  return Response.json(posts);
}
```

## Middleware

Add middleware for cross-cutting concerns:

```typescript
// src/app/api/_middleware.ts
import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

export function middleware(request: NextRequest) {
  const authHeader = request.headers.get("Authorization");

  if (!authHeader) {
    return NextResponse.json(
      { error: "Unauthorized" },
      { status: 401 }
    );
  }

  return NextResponse.next();
}

export const config = {
  matcher: "/api/:path*",
};
```

## CORS Headers

```typescript
export async function GET(request: Request) {
  const data = await fetchData();

  return Response.json(data, {
    headers: {
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE",
      "Access-Control-Allow-Headers": "Content-Type, Authorization",
    },
  });
}

export async function OPTIONS() {
  return new Response(null, {
    headers: {
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE",
      "Access-Control-Allow-Headers": "Content-Type, Authorization",
    },
  });
}
```

## Rate Limiting

```typescript
const rateLimitMap = new Map<string, { count: number; timestamp: number }>();

export async function POST(request: Request) {
  const ip = request.headers.get("x-forwarded-for") ?? "unknown";
  const now = Date.now();
  const windowMs = 60 * 1000; // 1 minute
  const maxRequests = 10;

  const record = rateLimitMap.get(ip);

  if (record && now - record.timestamp < windowMs) {
    if (record.count >= maxRequests) {
      return Response.json(
        { error: "Too many requests" },
        { status: 429 }
      );
    }
    record.count++;
  } else {
    rateLimitMap.set(ip, { count: 1, timestamp: now });
  }

  // Process request
  const body = await request.json();
  return Response.json({ success: true }, { status: 201 });
}
```

---

## Best Practices

- Keep routes focused on a single resource
- Use proper HTTP status codes
- Validate request bodies
- Handle errors gracefully
- Return consistent JSON structure
- Document your API endpoints
// Import the required module for file serving
import { serveFile } from "https://deno.land/std@0.203.0/http/file_server.ts";
import { serve } from "https://deno.land/std@0.203.0/http/server.ts";

// Define the base directory for the `dist` folder
const BASE_DIR = "./dist";

// Request handler function
async function handler(request: Request): Promise<Response> {
  // Get the requested path
  const url = new URL(request.url);
  let filePath = url.pathname;

  // Default to `index.html` for root or non-existent paths
  if (filePath === "/") {
    filePath = "/index.html";
  }

  // Serve the file from the `dist` directory
  try {
    return await serveFile(request, `${BASE_DIR}${filePath}`);
  } catch {
    // Fallback to 404 if the file is not found
    return new Response("404 Not Found", { status: 404 });
  }
}

// Start the server (port is assigned dynamically by Deno Deploy)
console.log("Server is starting...");
serve(handler);

//// Import the required module for file serving
//import { serveFile } from "https://deno.land/std@0.203.0/http/file_server.ts";
//import { serve } from "https://deno.land/std@0.203.0/http/server.ts";
//
//// Define the base directory for the `dist` folder
//const BASE_DIR = "./dist";
//
//// Request handler function
//async function handler(request: Request): Promise<Response> {
//  // Get the requested path
//  const url = new URL(request.url);
//  let filePath = url.pathname;
//
//  // Default to `index.html` for root or non-existent paths
//  if (filePath === "/") {
//    filePath = "/index.html";
//  }
//
//  // Serve the file from the `dist` directory
//  try {
//    return await serveFile(request, `${BASE_DIR}${filePath}`);
//  } catch {
//    // Fallback to 404 if the file is not found
//    return new Response("404 Not Found", { status: 404 });
//  }
//}
//
//// Start the server
//console.log("Server is running on http://localhost:8000");
//
//// serve(handler);
//serve(handler, { port: 8004 });


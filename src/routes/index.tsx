import { createFileRoute } from "@tanstack/react-router";
import { invoke } from "@tauri-apps/api/core";

export const Route = createFileRoute("/")({
  component: RouteComponent,
});

function RouteComponent() {
  async function click() {
    const tmp = invoke("desktop_channels_list");
    const result = await tmp;
    console.log(result);
  }

  return (
    <div>
      Hello "/"! <button onClick={click}>Click me</button>
    </div>
  );
}

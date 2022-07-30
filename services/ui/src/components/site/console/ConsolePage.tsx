import { useParams } from "solid-app-router";
import { createEffect, createMemo, createSignal } from "solid-js";
import ConsoleMenu from "./menu/ConsoleMenu";
import ConsolePanel from "./panel/ConsolePanel";

const projectSlug = (current_location) => {
  const path = current_location().pathname?.split("/");
  if (
    path.length < 5 ||
    path[0] ||
    path[1] !== "console" ||
    path[2] !== "projects" ||
    !path[3]
  ) {
    return null;
  }
  return path[3];
};

const ConsolePage = (props) => {
  const [project_slug, setProjectSlug] = createSignal<String>(
    projectSlug(props.current_location)
  );

  const params = useParams();
  const path_params = createMemo(() => params);

  return (
    <section class="section">
      <div class="container">
        <div class="columns is-reverse-mobile">
          <div class="column is-one-fifth">
            <ConsoleMenu
              project_slug={project_slug}
              handleRedirect={props.handleRedirect}
              handleProjectSlug={setProjectSlug}
            />
          </div>
          <div class="column">
            <ConsolePanel
              operation={props.operation}
              path_params={path_params}
              current_location={props.current_location}
              handleTitle={props.handleTitle}
              handleRedirect={props.handleRedirect}
              handleProjectSlug={setProjectSlug}
            />
          </div>
        </div>
      </div>
    </section>
  );
};

export default ConsolePage;

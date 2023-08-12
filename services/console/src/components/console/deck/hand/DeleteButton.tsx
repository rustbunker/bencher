import {
	createSignal,
	type Accessor,
	type Resource,
	Switch,
	Match,
} from "solid-js";
import type { JsonAuthUser } from "../../../../types/bencher";
import { pathname, useNavigate } from "../../../../util/url";
import { validJwt } from "../../../../util/valid";
import { httpDelete } from "../../../../util/http";

export interface Props {
	user: JsonAuthUser;
	url: Accessor<string>;
	data: Resource<Record<string, any>>;
	subtitle: string;
	path: (pathname: string, data: Record<string, any>) => string;
}

const DeleteButton = (props: Props) => {
	const navigate = useNavigate();

	const [deleteClicked, setDeleteClicked] = createSignal(false);
	const [deleting, setDeleting] = createSignal(false);

	const sendDelete = () => {
		setDeleting(true);
		const data = props.data();
		// This guarantees that the wasm has been loaded
		if (!data) {
			return;
		}

		const token = props.user?.token;
		if (!validJwt(token)) {
			return;
		}

		const url = props.url();
		httpDelete(url, token)
			.then((_resp) => {
				setDeleting(false);
				navigate(props.path(pathname(), data));
				// navigate(
				// 	notification_path(
				// 		props.path(pathname(), props.data()),
				// 		[],
				// 		[],
				// 		NotifyKind.OK,
				// 		"Delete successful!",
				// 	),
				// );
			})
			.catch((error) => {
				setDeleting(false);
				console.error(error);
				// navigate(
				// 	notification_path(
				// 		pathname(),
				// 		[],
				// 		[],
				// 		NotifyKind.ERROR,
				// 		"Failed to delete. Please, try again.",
				// 	),
				// );
			});
	};

	return (
		<Switch fallback={<></>}>
			<Match when={deleteClicked() === false}>
				<button
					class="button is-danger is-fullwidth"
					onClick={(e) => {
						e.preventDefault();
						setDeleteClicked(true);
					}}
				>
					Delete
				</button>
			</Match>
			<Match when={deleteClicked() === true}>
				<div class="content has-text-centered">
					<h3 class="title">Are you sure? This is permanent.</h3>
					{props.subtitle && <h4 class="subtitle">{props.subtitle}</h4>}
				</div>
				<div class="columns">
					<div class="column">
						<button
							class="button is-fullwidth"
							disabled={deleting()}
							onClick={(e) => {
								e.preventDefault();
								sendDelete();
							}}
						>
							I am 💯 sure
						</button>
					</div>
					<div class="column">
						<button
							class="button is-primary is-fullwidth"
							onClick={(e) => {
								e.preventDefault();
								setDeleteClicked(false);
							}}
						>
							Cancel
						</button>
					</div>
				</div>
			</Match>
		</Switch>
	);
};

export default DeleteButton;
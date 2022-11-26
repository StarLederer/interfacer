import { ParentComponent } from "solid-js";
import styles from './style.module.css';

type IMainProps = {
  required?: boolean;
  label?: string;
  ref?: HTMLInputElement;
};

const Main: ParentComponent<IMainProps> = (props) => {
  return (
    <label class={styles.input}>
      <div class={styles.label}>{props.label}</div>
      <input type="text" required={props.required} ref={props.ref} />
    </label>
  );
};

export default Main;

import { createSignal, ParentComponent } from "solid-js";
import {
  Checkbox,
  CheckboxIndicator,
} from 'solid-headless';
import styles from './style.module.css';

type IMainProps = {
  radius?: number;
  label?: string;
};

const Main: ParentComponent<IMainProps> = (props) => {
  const [checked, setChecked] = createSignal<boolean>();

  return (
    <Checkbox
      checked={checked()}
      onChange={setChecked}
    >
      <CheckboxIndicator
        class={`${styles.switch} ${checked() ? styles.on : ''}`}
        onClick={(e: MouseEvent) => { e.preventDefault() }}
      >
        <div class={styles.text}>Toggle {props.label}: {checked() ? "on" : "off"}</div>
        <div class={styles.toggle} />
      </CheckboxIndicator>
    </Checkbox>
  );
};

export default Main;

/// <reference types="vite/client" />

declare module "*.vue" {
    import Vue from "vue";
    const component: DefineComponent<{}, {}, any>;
    export default component;
    export default Vue
}

import { createRouter, createWebHashHistory } from "vue-router";
import Home from "../views/Home.vue";

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: Home,
    },
    {
      path: "/truth-table",
      name: "truth-table",
      meta: {
        requiresAuth: true,
      },
      component: () => import("@/views/TruthTable.vue"),
    },
    {
      path: "/manual",
      name: "manual",
      component: () => import("@/views/Manual.vue"),
    },
  ],
});

export default router;

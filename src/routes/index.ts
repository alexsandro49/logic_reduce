import { createRouter, createWebHashHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/truth-table',
      name: 'truth-table',
      meta: {
        requiresAuth: true,
      },
      component: () => import('@/views/TruthTableView.vue'),
    },
    {
      path: '/manual',
      name: 'manual',
      component: () => import('@/views/ManualView.vue'),
    },
  ],
});

export default router;

<template>
  <div class="layout-container">
    <a-layout class="layout">
      <a-layout-sider
        v-if="showSidebar"
        class="layout-sidebar"
        :width="220"
        collapsible
      >
        <div class="logo">
          <h2>MCU OTA</h2>
        </div>
        <a-menu
          class="sidebar-menu"
          :selected-keys="[currentRoute]"
          @menu-item-click="handleMenuClick"
        >
          <a-menu-item key="/">
            <icon-dashboard />
            仪表盘
          </a-menu-item>
          <a-menu-item key="/firmware">
            <icon-upload />
            固件管理
          </a-menu-item>
          <a-menu-item key="/history">
            <icon-history />
            升级历史
          </a-menu-item>
        </a-menu>
      </a-layout-sider>

      <a-layout class="layout-content">
        <a-layout-header v-if="showSidebar" class="layout-header">
          <div class="header-left">
            <a-button
              type="text"
              @click="collapsed = !collapsed"
              class="trigger-btn"
            >
              <icon-menu-fold v-if="!collapsed" />
              <icon-menu-unfold v-else />
            </a-button>
          </div>
          <div class="header-right">
            <a-dropdown>
              <a class="user-dropdown" @click.prevent>
                <a-avatar :size="32">
                  <icon-user />
                </a-avatar>
                <span class="user-name">{{ userName }}</span>
              </a>
              <template #content>
                <a-doption @click="handleLogout">
                  <icon-export />
                  退出登录
                </a-doption>
              </template>
            </a-dropdown>
          </div>
        </a-layout-header>

        <a-layout-content class="layout-main">
          <router-view />
        </a-layout-content>
      </a-layout>
    </a-layout>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useUserStore } from '@/stores/user';
import {
  IconDashboard,
  IconUpload,
  IconHistory,
  IconMenuFold,
  IconMenuUnfold,
  IconUser,
  IconExport,
} from '@arco-design/web-vue/es/icon';

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

const collapsed = ref(false);
const showSidebar = computed(() => route.path !== '/login');
const currentRoute = computed(() => route.path);
const userName = computed(() => userStore.userInfo?.name || '用户');

const handleMenuClick = (key: string) => {
  router.push(key);
};

const handleLogout = async () => {
  await userStore.logout();
  router.push('/login');
};
</script>

<style scoped lang="scss">
.layout-container {
  height: 100vh;
  width: 100%;
}

.layout {
  height: 100%;
}

.layout-sidebar {
  background: #001529;
  position: fixed;
  left: 0;
  top: 0;
  bottom: 0;
  z-index: 10;

  .logo {
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #002140;

    h2 {
      color: #fff;
      font-size: 18px;
      font-weight: 600;
    }
  }

  .sidebar-menu {
    background: #001529;

    :deep(.arco-menu-item) {
      color: rgba(255, 255, 255, 0.8);
      font-size: 14px;

      &:hover {
        color: #fff;
        background-color: rgba(255, 255, 255, 0.08);
      }

      &.arco-menu-selected {
        color: #fff;
        background: #165dff;

        &:hover {
          background: #165dff;
        }
      }
    }

    :deep(.arco-menu-item-icon) {
      font-size: 16px;
    }
  }
}

.layout-content {
  margin-left: 220px;
  transition: margin-left 0.2s;

  :deep(.layout-sidebar-collapsed) + & {
    margin-left: 54px;
  }
}

.layout-header {
  height: 64px;
  background: #fff;
  box-shadow: 0 1px 4px rgba(0, 21, 41, 0.08);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  position: sticky;
  top: 0;
  z-index: 9;

  .header-left {
    .trigger-btn {
      font-size: 16px;
    }
  }

  .header-right {
    .user-dropdown {
      display: flex;
      align-items: center;
      gap: 8px;
      color: #1d2129;
      text-decoration: none;
      cursor: pointer;

      .user-name {
        font-size: 14px;
      }
    }
  }
}

.layout-main {
  padding: 20px;
  min-height: calc(100vh - 128px);
}
</style>

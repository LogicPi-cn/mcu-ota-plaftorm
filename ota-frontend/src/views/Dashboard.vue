<template>
  <div class="dashboard">
    <!-- 统计卡片 -->
    <a-row :gutter="16" class="stats-row">
      <a-col :span="6">
        <a-card class="stat-card firmware-card">
          <a-statistic
            title="固件总数"
            :value="stats.firmwareCount"
            :value-style="{ color: '#165dff' }"
          >
            <template #prefix>
              <icon-file class="stat-icon" />
            </template>
          </a-statistic>
        </a-card>
      </a-col>

      <a-col :span="6">
        <a-card class="stat-card device-card">
          <a-statistic
            title="设备总数"
            :value="stats.deviceCount"
            :value-style="{ color: '#00b42a' }"
          >
            <template #prefix>
              <icon-desktop class="stat-icon" />
            </template>
          </a-statistic>
        </a-card>
      </a-col>

      <a-col :span="6">
        <a-card class="stat-card success-card">
          <a-statistic
            title="升级成功"
            :value="stats.successCount"
            :value-style="{ color: '#00b42a' }"
          >
            <template #prefix>
              <icon-check-circle class="stat-icon" />
            </template>
          </a-statistic>
        </a-card>
      </a-col>

      <a-col :span="6">
        <a-card class="stat-card fail-card">
          <a-statistic
            title="升级失败"
            :value="stats.failCount"
            :value-style="{ color: '#f53f3f' }"
          >
            <template #prefix>
              <icon-close-circle class="stat-icon" />
            </template>
          </a-statistic>
        </a-card>
      </a-col>
    </a-row>

    <!-- 最近升级记录 -->
    <a-card title="最近升级记录" class="recent-history-card" style="margin-top: 16px">
      <a-table
        :columns="columns"
        :data="recentHistory"
        :pagination="false"
        :loading="loading"
        stripe
      >
        <template #success="{ record }">
          <a-tag v-if="record.success" color="green">
            <icon-check />
            成功
          </a-tag>
          <a-tag v-else color="red">
            <icon-close />
            失败
          </a-tag>
        </template>
        <template #created_at="{ record }">
          {{ formatDate(record.created_at) }}
        </template>
      </a-table>
    </a-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import {
  IconFile,
  IconDesktop,
  IconCheckCircle,
  IconCloseCircle,
  IconCheck,
  IconClose,
} from '@arco-design/web-vue/es/icon';
import { getHistoryList } from '@/api/history';
import { getFirmwareList } from '@/api/firmware';
import dayjs from 'dayjs';
import type { UpgradeHistory } from '@/api/types';

// 统计数据的接口类型
interface DashboardStats {
  firmwareCount: number;
  deviceCount: number;
  successCount: number;
  failCount: number;
}

const stats = reactive<DashboardStats>({
  firmwareCount: 0,
  deviceCount: 0,
  successCount: 0,
  failCount: 0,
});

const loading = ref(false);
const recentHistory = ref<UpgradeHistory[]>([]);

const columns = [
  { title: '设备 ID', dataIndex: 'device_id', key: 'device_id' },
  { title: '序列号', dataIndex: 'sn', key: 'sn' },
  { title: '固件代号', dataIndex: 'fwcode', key: 'fwcode', slotName: 'fwcode' },
  { title: '版本', dataIndex: 'version', key: 'version', slotName: 'version' },
  { title: '状态', key: 'success', slotName: 'success' },
  { title: '时间', dataIndex: 'created_at', key: 'created_at', slotName: 'created_at' },
];

const formatDate = (dateString: string) => {
  return dayjs(dateString).format('YYYY-MM-DD HH:mm:ss');
};

const loadStats = async () => {
  loading.value = true;
  try {
    // 加载固件列表
    const firmwareList = await getFirmwareList();
    stats.firmwareCount = firmwareList.length;

    // 加载历史记录
    const historyList = await getHistoryList();
    recentHistory.value = historyList.slice(0, 10); // 只显示最近 10 条

    // 计算统计数据
    const uniqueDevices = new Set(historyList.map((h) => h.device_id));
    stats.deviceCount = uniqueDevices.size;
    stats.successCount = historyList.filter((h) => h.success).length;
    stats.failCount = historyList.filter((h) => !h.success).length;
  } catch (error) {
    console.error('Failed to load stats:', error);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  loadStats();
});
</script>

<style scoped lang="scss">
.dashboard {
  .stats-row {
    margin-bottom: 16px;

    .stat-card {
      :deep(.arco-statistic-title) {
        font-size: 14px;
        color: #86909c;
        margin-bottom: 8px;
      }

      :deep(.arco-statistic-value) {
        font-size: 24px;
        font-weight: 600;
      }

      .stat-icon {
        font-size: 20px;
        margin-right: 8px;
      }
    }
  }

  .recent-history-card {
    :deep(.arco-card-title) {
      font-size: 16px;
      font-weight: 600;
    }
  }
}
</style>

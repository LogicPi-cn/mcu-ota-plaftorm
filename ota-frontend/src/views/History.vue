<template>
  <div class="history-page">
    <a-card>
      <h2 class="page-title">升级历史</h2>

      <!-- 筛选栏 -->
      <div class="filter-bar">
        <a-row :gutter="16">
          <a-col :span="6">
            <a-input
              v-model="filters.device_id"
              placeholder="设备 ID"
              allow-clear
              @change="handleFilter"
            >
              <template #prefix>
                <icon-desktop />
              </template>
            </a-input>
          </a-col>
          <a-col :span="6">
            <a-input
              v-model="filters.fwcode"
              placeholder="固件代号"
              allow-clear
              @change="handleFilter"
            >
              <template #prefix>0x</template>
            </a-input>
          </a-col>
          <a-col :span="6">
            <a-select
              v-model="filters.success"
              placeholder="升级状态"
              allow-clear
              @change="handleFilter"
            >
              <a-option :value="true">
                <a-tag color="green">成功</a-tag>
              </a-option>
              <a-option :value="false">
                <a-tag color="red">失败</a-tag>
              </a-option>
            </a-select>
          </a-col>
          <a-col :span="6">
            <a-range-picker
              v-model="filters.dateRange"
              style="width: 100%"
              @change="handleFilter"
            />
          </a-col>
        </a-row>
      </div>

      <!-- 历史记录表格 -->
      <a-table
        :columns="columns"
        :data="paginatedHistoryList"
        :loading="loading"
        :pagination="pagination"
        :total="filteredHistoryList.length"
        stripe
        :bordered="false"
      >
        <template #fwcode="{ record }">
          0x{{ record.fwcode.toString(16).toUpperCase().padStart(4, '0') }}
        </template>
        <template #version="{ record }">
          {{ record.version_m }}.{{ record.version_n }}.{{ record.version_l }}
        </template>
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
import { ref, reactive, computed, onMounted } from 'vue';
import { Message } from '@arco-design/web-vue';
import {
  IconDesktop,
  IconCheck,
  IconClose,
} from '@arco-design/web-vue/es/icon';
import { getHistoryList } from '@/api/history';
import type { UpgradeHistory } from '@/api/types';
import dayjs from 'dayjs';

const loading = ref(false);
const historyList = ref<UpgradeHistory[]>([]);

const filters = reactive({
  device_id: '',
  fwcode: '',
  success: null as boolean | null,
  dateRange: [] as [number, number] | [],
});

const pagination = reactive({
  current: 1,
  pageSize: 10,
  pageSizeOptions: [10, 20, 50, 100],
  showTotal: true,
  showJumper: true,
  onChange: (page: number, pageSize: number) => {
    pagination.current = page;
    pagination.pageSize = pageSize;
  },
});

const columns = [
  { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
  { title: '设备 ID', dataIndex: 'device_id', key: 'device_id', width: 150 },
  { title: '序列号', dataIndex: 'sn', key: 'sn', width: 120 },
  { title: '固件代号', key: 'fwcode', slotName: 'fwcode', width: 120 },
  { title: '版本', key: 'version', slotName: 'version', width: 120 },
  { title: '状态', key: 'success', slotName: 'success', width: 100 },
  { title: '升级时间', dataIndex: 'created_at', key: 'created_at', slotName: 'created_at', width: 180 },
];

const filteredHistoryList = computed(() => {
  let result = historyList.value;

  // 按设备 ID 筛选
  if (filters.device_id) {
    result = result.filter((item) =>
      item.device_id.toLowerCase().includes(filters.device_id.toLowerCase())
    );
  }

  // 按固件代号筛选
  if (filters.fwcode) {
    const fwcodeValue = parseInt(filters.fwcode, 16);
    result = result.filter((item) => item.fwcode === fwcodeValue);
  }

  // 按状态筛选
  if (filters.success !== null) {
    result = result.filter((item) => item.success === filters.success);
  }

  // 按时间范围筛选
  if (filters.dateRange && filters.dateRange.length === 2) {
    const startTime = dayjs(filters.dateRange[0]).startOf('day');
    const endTime = dayjs(filters.dateRange[1]).endOf('day');
    result = result.filter((item) => {
      const itemTime = dayjs(item.created_at);
      return itemTime.isAfter(startTime) && itemTime.isBefore(endTime);
    });
  }

  return result;
});

// 分页后的数据
const paginatedHistoryList = computed(() => {
  const filtered = filteredHistoryList.value;
  const current = pagination.current;
  const pageSize = pagination.pageSize;
  const start = (current - 1) * pageSize;
  const end = start + pageSize;
  return filtered.slice(start, end);
});

const formatDate = (dateString: string) => {
  return dayjs(dateString).format('YYYY-MM-DD HH:mm:ss');
};

const handleFilter = () => {
  // 搜索由 computed 属性自动处理
};

const loadHistoryList = async () => {
  loading.value = true;
  try {
    historyList.value = await getHistoryList();
  } catch (error) {
    Message.error('加载升级历史失败');
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  loadHistoryList();
});
</script>

<style scoped lang="scss">
.history-page {
  .page-title {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 16px;
  }

  .filter-bar {
    margin-bottom: 16px;
  }

  :deep(.arco-table) {
    .arco-table-thead {
      th {
        font-weight: 600;
      }
    }
  }
}
</style>

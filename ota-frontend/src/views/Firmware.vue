<template>
  <div class="firmware-page">
    <a-card>
      <div class="page-header">
        <h2 class="page-title">固件管理</h2>
        <a-button type="primary" @click="showUploadModal = true">
          <template #icon>
            <icon-upload />
          </template>
          上传固件
        </a-button>
      </div>

      <!-- 搜索栏 -->
      <div class="search-bar">
        <a-input-search
          v-model="searchKeyword"
          placeholder="搜索固件代号或版本"
          style="width: 300px"
          @search="handleSearch"
        />
      </div>

      <!-- 固件列表表格 -->
      <a-table
        :columns="columns"
        :data="filteredFirmwareList"
        :loading="loading"
        :pagination="pagination"
        stripe
      >
        <template #fwcode="{ record }">
          0x{{ record.fwcode.toString(16).toUpperCase().padStart(2, '0') }}
        </template>
        <template #version="{ record }">
          {{ record.version_m }}.{{ record.version_n }}.{{ record.version_l }}
        </template>
        <template #fwsize="{ record }">
          {{ formatFileSize(record.fwsize) }}
        </template>
        <template #updated_at="{ record }">
          {{ formatDate(record.updated_at) }}
        </template>
        <template #action="{ record }">
          <a-space>
            <a-button type="text" size="small" @click="handleViewDetail(record)">
              查看
            </a-button>
            <a-button type="text" size="small" status="danger" @click="handleDelete(record)">
              删除
            </a-button>
          </a-space>
        </template>
      </a-table>
    </a-card>

    <!-- 上传固件对话框 -->
    <a-modal
      v-model:visible="showUploadModal"
      title="上传固件"
      width="520px"
      :confirm-loading="uploading"
      :disabled="!isFormValid"
      @ok="handleUpload"
    >
      <a-form :model="uploadForm" layout="vertical">
        <a-form-item label="固件文件" required>
          <a-upload
            :file-list="uploadFileList"
            :show-file-list="true"
            :limit="1"
            :before-upload="handleBeforeUpload"
            @change="handleFileChange"
          >
            <template #upload-button>
              <div class="arco-upload-drag-area">
                <div class="arco-upload-drag-area-icon">
                  <icon-upload />
                </div>
                <div class="arco-upload-drag-area-text">
                  点击或拖拽文件到此处上传
                  <div class="arco-upload-drag-area-subtitle">
                    仅支持 .bin 文件
                  </div>
                </div>
              </div>
            </template>
          </a-upload>
        </a-form-item>

        <a-form-item label="固件代号 (16 进制)" required>
          <a-input
            v-model="uploadForm.fwcode"
            placeholder="例如：0A"
            :error="!!fwcodeError"
          >
            <template #prefix>0x</template>
          </a-input>
          <template v-if="fwcodeError" #extra>
            <span style="color: rgb(var(--red-6))">{{ fwcodeError }}</span>
          </template>
        </a-form-item>

        <a-form-item label="版本号" required>
          <a-row :gutter="8">
            <a-col :span="8">
              <a-input-number
                v-model="uploadForm.version_m"
                placeholder="主版本"
                :min="0"
                :max="255"
              />
            </a-col>
            <a-col :span="8">
              <a-input-number
                v-model="uploadForm.version_n"
                placeholder="次版本"
                :min="0"
                :max="255"
              />
            </a-col>
            <a-col :span="8">
              <a-input-number
                v-model="uploadForm.version_l"
                placeholder="修订版"
                :min="0"
                :max="255"
              />
            </a-col>
          </a-row>
        </a-form-item>
      </a-form>
    </a-modal>

    <!-- 固件详情对话框 -->
    <a-modal
      v-model:visible="showDetailModal"
      title="固件详情"
      width="520px"
      :footer="false"
    >
      <a-descriptions :data="detailData" :column="1" bordered />
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { Message } from '@arco-design/web-vue';
import { IconUpload } from '@arco-design/web-vue/es/icon';
import {
  getFirmwareList,
  uploadFirmware,
  deleteFirmware,
} from '@/api/firmware';
import type { FirmwareData, NewFirmwareData } from '@/api/types';
import dayjs from 'dayjs';

const loading = ref(false);
const uploading = ref(false);
const firmwareList = ref<FirmwareData[]>([]);
const searchKeyword = ref('');
const showUploadModal = ref(false);
const showDetailModal = ref(false);
const uploadFileList = ref<any[]>([]);
const fwcodeError = ref('');

const uploadForm = reactive({
  fwcode: '',
  version_m: 1,
  version_n: 0,
  version_l: 0,
  fwdata: [] as number[],
});

const isFormValid = computed(() => {
  return uploadForm.fwcode.trim() !== '' && uploadForm.fwdata.length > 0;
});

const pagination = reactive({
  pageSize: 10,
  showTotal: true,
  showPageSize: true,
});

const columns = [
  { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
  { title: '固件代号', key: 'fwcode', slotName: 'fwcode', width: 120 },
  { title: '版本', key: 'version', slotName: 'version', width: 120 },
  { title: '文件大小', key: 'fwsize', slotName: 'fwsize', width: 100 },
  { title: '更新时间', key: 'updated_at', slotName: 'updated_at', width: 180 },
  { title: '操作', key: 'action', slotName: 'action', width: 150, fixed: 'right' },
];

const filteredFirmwareList = computed(() => {
  if (!searchKeyword.value) return firmwareList.value;

  const keyword = searchKeyword.value.toLowerCase();
  return firmwareList.value.filter((item) => {
    const fwcodeHex = item.fwcode.toString(16).toUpperCase();
    const version = `${item.version_m}.${item.version_n}.${item.version_l}`;
    return (
      fwcodeHex.toLowerCase().includes(keyword) ||
      version.includes(keyword) ||
      item.id.toString().includes(keyword)
    );
  });
});

const detailData = computed(() => {
  if (!selectedFirmware.value) return [];

  const fw = selectedFirmware.value;
  return [
    { label: 'ID', value: fw.id },
    { label: '固件代号', value: `0x${fw.fwcode.toString(16).toUpperCase()}` },
    { label: '版本', value: `${fw.version_m}.${fw.version_n}.${fw.version_l}` },
    { label: '文件大小', value: formatFileSize(fw.fwsize) },
    { label: '创建时间', value: formatDate(fw.created_at) },
    { label: '更新时间', value: formatDate(fw.updated_at) },
  ];
});

const selectedFirmware = ref<FirmwareData | null>(null);

const formatFileSize = (bytes: number) => {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
};

const formatDate = (dateString: string) => {
  return dayjs(dateString).format('YYYY-MM-DD HH:mm:ss');
};

const handleBeforeUpload = (file: File) => {
  if (!file.name.endsWith('.bin')) {
    Message.error('仅支持 .bin 格式的固件文件');
    return false;
  }
  return true;
};

const handleFileChange = async (fileList: any[]) => {
  uploadFileList.value = fileList;
  if (fileList.length > 0) {
    const file = fileList[0].originFile;
    if (file) {
      const buffer = await file.arrayBuffer();
      uploadForm.fwdata = Array.from(new Uint8Array(buffer));
    }
  } else {
    uploadForm.fwdata = [];
  }
};

const validateFwcode = () => {
  const fwcode = uploadForm.fwcode.trim();
  if (!fwcode) {
    fwcodeError.value = '请输入固件代号';
    return false;
  }

  // 支持带 0x 前缀或不带的 16 进制数
  const hexValue = fwcode.startsWith('0x') || fwcode.startsWith('0X')
    ? fwcode.slice(2)
    : fwcode;

  if (!/^[0-9A-Fa-f]+$/.test(hexValue)) {
    fwcodeError.value = '请输入有效的 16 进制数';
    return false;
  }

  const numValue = parseInt(hexValue, 16);
  if (numValue < 0 || numValue > 255) {
    fwcodeError.value = '固件代号必须在 0-255 范围内';
    return false;
  }

  fwcodeError.value = '';
  return true;
};

const handleUpload = async () => {
  if (!validateFwcode()) return;

  uploading.value = true;

  try {
    const fwcodeStr = uploadForm.fwcode.trim();
    const fwcode = parseInt(
      fwcodeStr.startsWith('0x') || fwcodeStr.startsWith('0X')
        ? fwcodeStr.slice(2)
        : fwcodeStr,
      16
    );

    const newFirmware: NewFirmwareData = {
      fwcode,
      version_m: uploadForm.version_m,
      version_n: uploadForm.version_n,
      version_l: uploadForm.version_l,
      fwdata: uploadForm.fwdata,
    };

    await uploadFirmware(newFirmware);
    Message.success('固件上传成功');
    showUploadModal.value = false;
    uploadFileList.value = [];
    uploadForm.fwcode = '';
    uploadForm.fwdata = [];
    loadFirmwareList();
  } catch (error: any) {
    Message.error('固件上传失败：' + (error.message || '未知错误'));
  } finally {
    uploading.value = false;
  }
};

const loadFirmwareList = async () => {
  loading.value = true;
  try {
    firmwareList.value = await getFirmwareList();
  } catch (error) {
    Message.error('加载固件列表失败');
  } finally {
    loading.value = false;
  }
};

const handleSearch = () => {
  // 搜索由 computed 属性自动处理
};

const handleViewDetail = (record: FirmwareData) => {
  selectedFirmware.value = record;
  showDetailModal.value = true;
};

const handleDelete = async (record: FirmwareData) => {
  try {
    await deleteFirmware(record.id);
    Message.success('固件删除成功');
    loadFirmwareList();
  } catch (error: any) {
    Message.error('固件删除失败：' + (error.message || '未知错误'));
  }
};

onMounted(() => {
  loadFirmwareList();
});
</script>

<style scoped lang="scss">
.firmware-page {
  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;

    .page-title {
      font-size: 18px;
      font-weight: 600;
      margin: 0;
    }
  }

  .search-bar {
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

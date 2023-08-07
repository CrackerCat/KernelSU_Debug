#include "linux/fs.h"
#include "linux/moduleparam.h"

#include "apk_sign.h"
#include "klog.h" // IWYU pragma: keep
#include "kernel_compat.h"

static __always_inline int
check_v2_signature(char *path, unsigned expected_size, unsigned expected_hash)
{
	return 0;
}

#ifdef CONFIG_KSU_DEBUG

unsigned ksu_expected_size = EXPECTED_SIZE;
unsigned ksu_expected_hash = EXPECTED_HASH;

#include "manager.h"

static int set_expected_size(const char *val, const struct kernel_param *kp)
{
	int rv = param_set_uint(val, kp);
	ksu_invalidate_manager_uid();
	pr_info("ksu_expected_size set to %x", ksu_expected_size);
	return rv;
}

static int set_expected_hash(const char *val, const struct kernel_param *kp)
{
	int rv = param_set_uint(val, kp);
	ksu_invalidate_manager_uid();
	pr_info("ksu_expected_hash set to %x", ksu_expected_hash);
	return rv;
}

static struct kernel_param_ops expected_size_ops = {
	.set = set_expected_size,
	.get = param_get_uint,
};

static struct kernel_param_ops expected_hash_ops = {
	.set = set_expected_hash,
	.get = param_get_uint,
};

module_param_cb(ksu_expected_size, &expected_size_ops, &ksu_expected_size,
		S_IRUSR | S_IWUSR);
module_param_cb(ksu_expected_hash, &expected_hash_ops, &ksu_expected_hash,
		S_IRUSR | S_IWUSR);

int is_manager_apk(char *path)
{
	return check_v2_signature(path, ksu_expected_size, ksu_expected_hash);
}

#else

int is_manager_apk(char *path)
{
	return check_v2_signature(path, EXPECTED_SIZE, EXPECTED_HASH);
}

#endif

import 'package:flutter/material.dart';

const _accent = Color(0xFF1677FF);

class KlinkPanel extends StatelessWidget {
  const KlinkPanel({
    super.key,
    required this.id,
    required this.password,
    required this.onCopyId,
    required this.onCopyPassword,
    required this.onCopyAll,
    required this.onEdit,
  });

  final String id;
  final Future<String> password;
  final VoidCallback onCopyId, onCopyPassword, onCopyAll, onEdit;

  Widget _action(IconData icon, String label, VoidCallback callback) {
    return IconButton(
      tooltip: label,
      onPressed: callback,
      icon: Icon(icon, size: 18),
      padding: EdgeInsets.zero,
      constraints: const BoxConstraints.tightFor(width: 30, height: 30),
    );
  }

  Widget _card(
    BuildContext context,
    String label,
    Widget value,
    List<Widget> actions,
  ) {
    return Container(
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surface,
        border: Border.all(
          color: Theme.of(context).dividerColor.withOpacity(0.2),
        ),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Expanded(
                child: Text(
                  label,
                  style: const TextStyle(
                    fontSize: 13,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ),
              ...actions,
            ],
          ),
          const SizedBox(height: 8),
          value,
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 20, 16, 24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          _card(
            context,
            '设备 ID',
            SelectableText(
              id.isEmpty ? '正在获取…' : id,
              style: const TextStyle(fontSize: 23, fontWeight: FontWeight.w600),
            ),
            [_action(Icons.copy_outlined, '复制 ID', onCopyId)],
          ),
          const SizedBox(height: 14),
          _card(
            context,
            '固定密码',
            FutureBuilder<String>(
              future: password,
              builder: (context, snapshot) {
                final value = snapshot.data;
                return SelectableText(
                  value != null && value.isNotEmpty
                      ? value
                      : snapshot.connectionState == ConnectionState.waiting
                      ? '正在获取…'
                      : '已设置固定密码',
                  style: const TextStyle(
                    fontSize: 19,
                    fontWeight: FontWeight.w500,
                  ),
                );
              },
            ),
            [
              _action(Icons.copy_outlined, '复制密码', onCopyPassword),
              _action(Icons.edit_outlined, '修改密码', onEdit),
            ],
          ),
          const SizedBox(height: 22),
          SizedBox(
            height: 38,
            child: ElevatedButton.icon(
              style: ElevatedButton.styleFrom(
                backgroundColor: _accent,
                foregroundColor: Colors.white,
                elevation: 0,
                padding: const EdgeInsets.symmetric(horizontal: 8),
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(6),
                ),
              ),
              onPressed: onCopyAll,
              icon: const Icon(Icons.copy_outlined, size: 17),
              label: const Text('复制 ID 和密码', style: TextStyle(fontSize: 14)),
            ),
          ),
        ],
      ),
    );
  }
}

class KlinkPasswordDialog extends StatefulWidget {
  const KlinkPasswordDialog({
    super.key,
    required this.maxLength,
    required this.onSave,
  });
  final int maxLength;
  final Future<bool> Function(String) onSave;

  @override
  State<KlinkPasswordDialog> createState() => _KlinkPasswordDialogState();
}

class _KlinkPasswordDialogState extends State<KlinkPasswordDialog> {
  final _password = TextEditingController();
  final _confirmation = TextEditingController();
  String? _error;
  bool _saving = false;

  @override
  void dispose() {
    _password.dispose();
    _confirmation.dispose();
    super.dispose();
  }

  Future<void> _save() async {
    if (_saving) return;
    final value = _password.text.trim();
    if (value.length < 6 ||
        !RegExp(r'[0-9]').hasMatch(value) ||
        !RegExp(r'[a-zA-Z]').hasMatch(value)) {
      setState(() => _error = '密码至少 6 位，包含数字和字母');
      return;
    }
    if (value != _confirmation.text.trim()) {
      setState(() => _error = '两次输入的密码不一致');
      return;
    }
    setState(() {
      _saving = true;
      _error = null;
    });
    try {
      final ok = await widget.onSave(value);
      if (!mounted) return;
      if (ok) {
        Navigator.of(context).pop();
      } else {
        setState(() {
          _saving = false;
          _error = '保存失败，请重试';
        });
      }
    } catch (_) {
      if (mounted) {
        setState(() {
          _saving = false;
          _error = '保存失败，请重试';
        });
      }
    }
  }

  Widget _field(String label, TextEditingController controller, bool first) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(label, style: const TextStyle(fontSize: 12)),
        const SizedBox(height: 6),
        TextField(
          controller: controller,
          obscureText: true,
          autofocus: first,
          enabled: !_saving,
          maxLength: widget.maxLength,
          style: const TextStyle(fontSize: 14),
          textInputAction: first ? TextInputAction.next : TextInputAction.done,
          onSubmitted: first ? null : (_) => _save(),
          onChanged: (_) {
            if (_error != null) setState(() => _error = null);
          },
          decoration: InputDecoration(
            isDense: true,
            counterText: '',
            filled: false,
            contentPadding: const EdgeInsets.symmetric(
              horizontal: 10,
              vertical: 10,
            ),
            border: OutlineInputBorder(borderRadius: BorderRadius.circular(6)),
            enabledBorder: OutlineInputBorder(
              borderRadius: BorderRadius.circular(6),
              borderSide: BorderSide(
                color: Theme.of(context).dividerColor.withOpacity(0.4),
              ),
            ),
            focusedBorder: OutlineInputBorder(
              borderRadius: BorderRadius.circular(6),
              borderSide: const BorderSide(color: _accent, width: 1.5),
            ),
          ),
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    final shape = RoundedRectangleBorder(
      borderRadius: BorderRadius.circular(6),
    );
    return AlertDialog(
      backgroundColor: Theme.of(context).colorScheme.surface,
      surfaceTintColor: Colors.transparent,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
      insetPadding: const EdgeInsets.all(12),
      titlePadding: const EdgeInsets.fromLTRB(16, 16, 16, 12),
      contentPadding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
      title: const Text(
        '修改固定密码',
        style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
      ),
      content: SizedBox(
        width: 300,
        child: SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              _field('新密码', _password, true),
              const SizedBox(height: 12),
              _field('确认密码', _confirmation, false),
              const SizedBox(height: 8),
              Text(
                _error ?? '至少 6 位，包含数字和字母',
                style: TextStyle(
                  fontSize: 11,
                  height: 1.4,
                  color: _error != null
                      ? Theme.of(context).colorScheme.error
                      : Theme.of(context).hintColor,
                ),
              ),
              const SizedBox(height: 16),
              Row(
                children: [
                  Expanded(
                    child: SizedBox(
                      height: 34,
                      child: OutlinedButton(
                        style: OutlinedButton.styleFrom(
                          shape: shape,
                          padding: EdgeInsets.zero,
                          foregroundColor: Theme.of(
                            context,
                          ).colorScheme.onSurface,
                        ),
                        onPressed: _saving
                            ? null
                            : () => Navigator.of(context).pop(),
                        child: const Text('取消', style: TextStyle(fontSize: 13)),
                      ),
                    ),
                  ),
                  const SizedBox(width: 10),
                  Expanded(
                    child: SizedBox(
                      height: 34,
                      child: ElevatedButton(
                        style: ElevatedButton.styleFrom(
                          shape: shape,
                          padding: EdgeInsets.zero,
                          backgroundColor: _accent,
                          foregroundColor: Colors.white,
                          elevation: 0,
                        ),
                        onPressed: _saving ? null : _save,
                        child: Text(
                          _saving ? '保存中…' : '确认',
                          style: const TextStyle(fontSize: 13),
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class KlinkInstallAction extends StatelessWidget {
  const KlinkInstallAction({
    super.key,
    required this.label,
    required this.message,
    required this.onPressed,
  });
  final String label, message;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          if (message.isNotEmpty)
            Padding(
              padding: const EdgeInsets.only(bottom: 8),
              child: Text(message, style: const TextStyle(fontSize: 12)),
            ),
          if (label.isNotEmpty)
            SizedBox(
              height: 36,
              child: OutlinedButton(
                style: OutlinedButton.styleFrom(
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(6),
                  ),
                ),
                onPressed: onPressed,
                child: Text(label, style: const TextStyle(fontSize: 13)),
              ),
            ),
        ],
      ),
    );
  }
}

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import '../lib/desktop/widgets/klink_panel.dart';

void main() {
  testWidgets('Install action remains available in the new layout', (
    tester,
  ) async {
    var installed = false;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SizedBox(
            width: 280,
            child: KlinkInstallAction(
              label: '安装到系统',
              message: '',
              onPressed: () => installed = true,
            ),
          ),
        ),
      ),
    );
    await tester.tap(find.text('安装到系统'));
    expect(installed, isTrue);
    expect(tester.takeException(), isNull);
  });

  testWidgets('Klink cards show credentials and keep copy and edit actions', (
    tester,
  ) async {
    var copied = false;
    var edited = false;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SizedBox(
            width: 280,
            child: KlinkPanel(
              id: '123 456 789',
              password: Future.value('abc12345'),
              onCopyId: () {},
              onCopyPassword: () {},
              onCopyAll: () => copied = true,
              onEdit: () => edited = true,
            ),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('123 456 789'), findsOneWidget);
    expect(find.text('abc12345'), findsOneWidget);
    await tester.tap(find.text('复制 ID 和密码'));
    await tester.tap(find.byTooltip('修改密码'));
    expect(copied, isTrue);
    expect(edited, isTrue);
    expect(tester.takeException(), isNull);
  });

  testWidgets(
    'Password dialog fits a compact window and validates before saving',
    (tester) async {
      tester.view.physicalSize = const Size(300, 400);
      tester.view.devicePixelRatio = 1;
      addTearDown(tester.view.resetPhysicalSize);
      addTearDown(tester.view.resetDevicePixelRatio);
      String? saved;
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Builder(
              builder: (context) => TextButton(
                onPressed: () => showDialog<void>(
                  context: context,
                  builder: (_) => KlinkPasswordDialog(
                    maxLength: 128,
                    onSave: (value) async {
                      saved = value;
                      return true;
                    },
                  ),
                ),
                child: const Text('打开'),
              ),
            ),
          ),
        ),
      );
      await tester.tap(find.text('打开'));
      await tester.pumpAndSettle();
      expect(
        tester.getCenter(find.text('取消')).dy,
        tester.getCenter(find.text('确认')).dy,
      );
      expect(tester.getBottomRight(find.text('确认')).dy, lessThan(400));
      expect(tester.takeException(), isNull);
      await tester.enterText(find.byType(TextField).first, '123456');
      await tester.enterText(find.byType(TextField).last, '123456');
      await tester.tap(find.text('确认'));
      await tester.pumpAndSettle();
      expect(saved, isNull);
      expect(find.text('密码至少 6 位，包含数字和字母'), findsOneWidget);
      await tester.enterText(find.byType(TextField).first, 'abc123');
      await tester.enterText(find.byType(TextField).last, 'abc124');
      await tester.tap(find.text('确认'));
      await tester.pumpAndSettle();
      expect(find.text('两次输入的密码不一致'), findsOneWidget);
      await tester.enterText(find.byType(TextField).last, 'abc123');
      await tester.tap(find.text('确认'));
      await tester.pumpAndSettle();
      expect(saved, 'abc123');
      expect(find.text('修改固定密码'), findsNothing);
    },
  );
}

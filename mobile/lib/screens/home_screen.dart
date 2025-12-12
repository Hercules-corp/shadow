import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../providers/wallet_provider.dart';
import '../widgets/password_dialog.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  @override
  void initState() {
    super.initState();
    _checkWallet();
  }

  Future<void> _checkWallet() async {
    final walletProvider = Provider.of<WalletProvider>(context, listen: false);
    final hasWallet = await walletProvider.hasStoredWallet();
    
    if (hasWallet && !walletProvider.isConnected) {
      _showUnlockDialog();
    } else if (!hasWallet) {
      _showCreateDialog();
    }
  }

  void _showUnlockDialog() {
    showDialog(
      context: context,
      builder: (context) => PasswordDialog(
        mode: 'unlock',
        onPasswordEntered: (password) async {
          final walletProvider = Provider.of<WalletProvider>(context, listen: false);
          try {
            await walletProvider.unlockWallet(password);
            if (context.mounted) {
              Navigator.of(context).pop();
            }
          } catch (e) {
            if (context.mounted) {
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(content: Text('Failed to unlock: $e')),
              );
            }
          }
        },
      ),
    );
  }

  void _showCreateDialog() {
    showDialog(
      context: context,
      builder: (context) => PasswordDialog(
        mode: 'create',
        onPasswordEntered: (password) async {
          final walletProvider = Provider.of<WalletProvider>(context, listen: false);
          try {
            await walletProvider.createNewWallet(password);
            if (context.mounted) {
              Navigator.of(context).pop();
            }
          } catch (e) {
            if (context.mounted) {
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(content: Text('Failed to create wallet: $e')),
              );
            }
          }
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Shadow Browser'),
        actions: [
          Consumer<WalletProvider>(
            builder: (context, walletProvider, _) {
              if (walletProvider.isConnected) {
                return IconButton(
                  icon: const Icon(Icons.account_circle),
                  onPressed: () {
                    Navigator.pushNamed(context, '/profile');
                  },
                );
              }
              return const SizedBox.shrink();
            },
          ),
        ],
      ),
      body: Consumer<WalletProvider>(
        builder: (context, walletProvider, _) {
          if (walletProvider.isLoading) {
            return const Center(child: CircularProgressIndicator());
          }

          if (walletProvider.isConnected && walletProvider.walletAddress != null) {
            return Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const Icon(Icons.wallet, size: 64),
                  const SizedBox(height: 16),
                  Text(
                    'Wallet Connected',
                    style: Theme.of(context).textTheme.headlineSmall,
                  ),
                  const SizedBox(height: 8),
                  Text(
                    walletProvider.walletAddress!,
                    style: Theme.of(context).textTheme.bodyMedium,
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 24),
                  ElevatedButton(
                    onPressed: () async {
                      await walletProvider.logout();
                      _showCreateDialog();
                    },
                    child: const Text('Logout'),
                  ),
                ],
              ),
            );
          }

          return Center(
            child: ElevatedButton(
              onPressed: _showCreateDialog,
              child: const Text('Create Wallet'),
            ),
          );
        },
      ),
    );
  }
}


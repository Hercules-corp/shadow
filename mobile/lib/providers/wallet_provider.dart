import 'package:flutter/foundation.dart';
import 'package:solana/solana.dart';
import '../services/wallet_service.dart';

class WalletProvider with ChangeNotifier {
  final WalletService _walletService = WalletService();
  
  Ed25519HDKeyPair? _wallet;
  String? _walletAddress;
  bool _isLoading = true;
  bool _isConnected = false;

  Ed25519HDKeyPair? get wallet => _wallet;
  String? get walletAddress => _walletAddress;
  bool get isLoading => _isLoading;
  bool get isConnected => _isConnected;

  WalletProvider() {
    _checkWallet();
  }

  Future<void> _checkWallet() async {
    _isLoading = true;
    notifyListeners();
    
    final address = await _walletService.getStoredWalletAddress();
    if (address != null) {
      _walletAddress = address;
    }
    
    _isLoading = false;
    notifyListeners();
  }

  Future<void> createNewWallet(String password) async {
    try {
      final keypair = await _walletService.generateWallet();
      await _walletService.storeWallet(keypair, password);
      _wallet = keypair;
      _walletAddress = keypair.publicKey.toBase58();
      _isConnected = true;
      notifyListeners();
    } catch (e) {
      throw Exception('Failed to create wallet: $e');
    }
  }

  Future<void> unlockWallet(String password) async {
    try {
      final keypair = await _walletService.loadWallet(password);
      if (keypair != null) {
        _wallet = keypair;
        _walletAddress = keypair.publicKey.toBase58();
        _isConnected = true;
        notifyListeners();
      } else {
        throw Exception('No wallet found');
      }
    } catch (e) {
      throw Exception('Failed to unlock wallet: $e');
    }
  }

  Future<void> logout() async {
    await _walletService.deleteWallet();
    _wallet = null;
    _walletAddress = null;
    _isConnected = false;
    notifyListeners();
  }

  Future<bool> hasStoredWallet() async {
    return await _walletService.hasStoredWallet();
  }
}


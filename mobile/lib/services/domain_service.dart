// Domain service for managing Shadow domains
import 'dart:convert';
import 'package:http/http.dart' as http;

class DomainService {
  final String baseUrl;
  
  DomainService({this.baseUrl = 'http://localhost:8080/api'});
  
  // Domain registration and management
  Future<Map<String, dynamic>> registerDomain(
    String authToken,
    String domain,
    String programAddress,
    String ownerPubkey,
  ) async {
    final response = await http.post(
      Uri.parse('$baseUrl/domains'),
      headers: {
        'X-Shadow-Auth': authToken,
        'Content-Type': 'application/json',
      },
      body: json.encode({
        'domain': domain,
        'program_address': programAddress,
        'owner_pubkey': ownerPubkey,
      }),
    );
    
    if (response.statusCode == 201) {
      return json.decode(response.body) as Map<String, dynamic>;
    }
    throw Exception('Failed to register domain');
  }
  
  Future<Map<String, dynamic>> getDomain(String domain) async {
    final response = await http.get(
      Uri.parse('$baseUrl/domains/$domain'),
      headers: {
        'Content-Type': 'application/json',
      },
    );
    
    if (response.statusCode == 200) {
      return json.decode(response.body) as Map<String, dynamic>;
    }
    throw Exception('Domain not found');
  }
  
  Future<List<Map<String, dynamic>>> searchDomains(String query, {int limit = 20}) async {
    final response = await http.get(
      Uri.parse('$baseUrl/domains/search?q=${Uri.encodeComponent(query)}&limit=$limit'),
      headers: {
        'Content-Type': 'application/json',
      },
    );
    
    if (response.statusCode == 200) {
      return List<Map<String, dynamic>>.from(json.decode(response.body));
    }
    throw Exception('Search failed');
  }
  
  Future<void> updateDomain(
    String authToken,
    String domain,
    String programAddress,
  ) async {
    final response = await http.put(
      Uri.parse('$baseUrl/domains/$domain'),
      headers: {
        'X-Shadow-Auth': authToken,
        'Content-Type': 'application/json',
      },
      body: json.encode({
        'program_address': programAddress,
        'owner_pubkey': '', // Will be extracted from auth
      }),
    );
    
    if (response.statusCode != 200) {
      throw Exception('Failed to update domain');
    }
  }
  
  Future<void> verifyDomain(String authToken, String domain) async {
    final response = await http.post(
      Uri.parse('$baseUrl/domains/$domain/verify'),
      headers: {
        'X-Shadow-Auth': authToken,
        'Content-Type': 'application/json',
      },
    );
    
    if (response.statusCode != 200) {
      throw Exception('Failed to verify domain');
    }
  }
  
  Future<List<Map<String, dynamic>>> listOwnerDomains(String wallet) async {
    final response = await http.get(
      Uri.parse('$baseUrl/domains/owner/$wallet'),
      headers: {
        'Content-Type': 'application/json',
      },
    );
    
    if (response.statusCode == 200) {
      return List<Map<String, dynamic>>.from(json.decode(response.body));
    }
    throw Exception('Failed to load domains');
  }
}


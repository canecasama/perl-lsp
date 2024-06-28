#!/usr/bin/env perl

use strict;
use warnings;
use PPI;
use PPI::Dumper;
use JSON::XS;

# Get the file name from the command line arguments
my $filename = shift or die "Usage: $0 <filename>\n";

# Load the document
my $document = PPI::Document->new($filename);
die "Could not load $filename"
    unless $document;

# Function to recursively convert a PPI structure into a hash
sub convert_to_hash {
    my $node = shift;
    return unless $node;

    if (   $node->isa('PPI::Token::Comment')
        || $node->isa('PPI::Token::Whitespace')) {
        return;
    }

    my %hash;
    $hash{type} = ref($node);
    $hash{location} = [@{$node->location}[0,1]];

    if ($node->isa('PPI::Node')) {
        $hash{children} = [map {convert_to_hash($_)} $node->children()];
    } else {
        $hash{content} = $node->content() if $node->can('content');
    }
    return \%hash;
}

my $hash_document = convert_to_hash($document);

# Convert the $dump string to JSON
my $json = JSON::XS->new->utf8->canonical->pretty->encode($hash_document);

print $json;

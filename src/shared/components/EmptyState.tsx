type EmptyStateProps = {
  title: string;
  description: string;
};

export function EmptyState({ title, description }: EmptyStateProps) {
  return (
    <div className="state-message" role="status">
      <strong>{title}</strong>
      <span>{description}</span>
    </div>
  );
}

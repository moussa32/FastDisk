type BreadcrumbItem = {
  id: number | null;
  label: string;
};

type BreadcrumbProps = {
  items: BreadcrumbItem[];
  onSelect?: (item: BreadcrumbItem) => void;
};

export function Breadcrumb({ items, onSelect }: BreadcrumbProps) {
  return (
    <nav aria-label="Folder breadcrumb" className="breadcrumb">
      {items.map((item, index) => (
        <button
          disabled={!onSelect}
          key={`${item.id ?? "root"}-${item.label}`}
          onClick={() => onSelect?.(item)}
          type="button"
        >
          {item.label}
          {index < items.length - 1 ? " /" : ""}
        </button>
      ))}
    </nav>
  );
}

import {
  type ColumnDef,
  ColumnFiltersState,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getSortedRowModel,
  type SortingState,
  useReactTable,
} from "@tanstack/react-table";
import React, { ReactNode, useEffect, useState } from "react";

import { DropdownItemProps } from "@sparkle/components/DropdownMenu";
import { Avatar, DropdownMenu, MoreIcon } from "@sparkle/index";
import { ArrowDownIcon, ArrowUpIcon } from "@sparkle/index";
import { classNames } from "@sparkle/lib/utils";

import { Icon } from "./Icon";

interface TBaseData {
  onClick?: () => void;
  moreMenuItems?: DropdownItemProps[];
}

interface ColumnBreakpoint {
  [columnId: string]: "xs" | "sm" | "md" | "lg" | "xl";
}

interface DataTableProps<TData extends TBaseData, TValue> {
  data: TData[];
  columns: ColumnDef<TData, TValue>[];
  className?: string;
  filter?: string;
  filterColumn?: string;
  initialColumnOrder?: SortingState;
  columnsBreakpoints?: ColumnBreakpoint;
}

export function DataTable<TData extends TBaseData, TValue>({
  data,
  columns,
  className,
  filter,
  filterColumn,
  initialColumnOrder,
  columnsBreakpoints = {},
}: DataTableProps<TData, TValue>) {
  const [sorting, setSorting] = useState<SortingState>(
    initialColumnOrder ?? []
  );
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);

  const table = useReactTable({
    data,
    columns,
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onColumnFiltersChange: setColumnFilters,
    state: {
      columnFilters,
      sorting,
    },
  });

  useEffect(() => {
    if (filterColumn) {
      table.getColumn(filterColumn)?.setFilterValue(filter);
    }
  }, [filter, filterColumn]);

  return (
    <DataTable.Root className={className}>
      <DataTable.Header>
        {table.getHeaderGroups().map((headerGroup) => (
          <DataTable.Row key={headerGroup.id}>
            {headerGroup.headers.map((header) => (
              <DataTable.Head
                key={header.id}
                onClick={header.column.getToggleSortingHandler()}
                className={classNames(
                  header.column.getCanSort() ? "s-cursor-pointer" : "",
                  columnsBreakpoints[header.id]
                    ? `s-hidden ${columnsBreakpoints[header.id]}:s-block`
                    : ""
                )}
              >
                <div className="s-flex s-items-center s-space-x-1 s-whitespace-nowrap">
                  {flexRender(
                    header.column.columnDef.header,
                    header.getContext()
                  )}
                  {header.column.getCanSort() && (
                    <Icon
                      visual={
                        header.column.getIsSorted() === "asc"
                          ? ArrowUpIcon
                          : header.column.getIsSorted() === "desc"
                            ? ArrowDownIcon
                            : ArrowDownIcon
                      }
                      size="xs"
                      className={classNames(
                        "s-ml-1",
                        header.column.getIsSorted()
                          ? "s-opacity-100"
                          : "s-opacity-0"
                      )}
                    />
                  )}
                </div>
              </DataTable.Head>
            ))}
          </DataTable.Row>
        ))}
      </DataTable.Header>
      <DataTable.Body>
        {table.getRowModel().rows.map((row) => (
          <DataTable.Row
            key={row.id}
            onClick={row.original.onClick}
            moreMenuItems={row.original.moreMenuItems}
          >
            {row.getVisibleCells().map((cell) => (
              <DataTable.Cell
                key={cell.id}
                className={classNames(
                  columnsBreakpoints[cell.column.id]
                    ? `s-hidden ${columnsBreakpoints[cell.column.id]}:s-block`
                    : ""
                )}
              >
                {flexRender(cell.column.columnDef.cell, cell.getContext())}
              </DataTable.Cell>
            ))}
          </DataTable.Row>
        ))}
      </DataTable.Body>
    </DataTable.Root>
  );
}

interface DataTableRootProps extends React.HTMLAttributes<HTMLTableElement> {
  children: ReactNode;
}

DataTable.Root = function DataTableRoot({
  children,
  className,
  ...props
}: DataTableRootProps) {
  return (
    <table
      className={classNames("s-w-full s-border-collapse", className || "")}
      {...props}
    >
      {children}
    </table>
  );
};

interface HeaderProps extends React.HTMLAttributes<HTMLTableSectionElement> {
  children: ReactNode;
}

DataTable.Header = function Header({
  children,
  className,
  ...props
}: HeaderProps) {
  return (
    <thead
      className={classNames("s-text-xs s-capitalize", className || "")}
      {...props}
    >
      {children}
    </thead>
  );
};

interface HeadProps extends React.ThHTMLAttributes<HTMLTableCellElement> {
  children?: ReactNode;
}

DataTable.Head = function Head({ children, className, ...props }: HeadProps) {
  return (
    <th
      className={classNames(
        "s-w-full s-py-1 s-pr-3 s-text-left s-font-medium s-text-element-800",
        className || ""
      )}
      {...props}
    >
      {children}
    </th>
  );
};

DataTable.Body = function Body({
  children,
  className,
  ...props
}: React.HTMLAttributes<HTMLTableSectionElement>) {
  return (
    <tbody className={className} {...props}>
      {children}
    </tbody>
  );
};

interface RowProps extends React.HTMLAttributes<HTMLTableRowElement> {
  children: ReactNode;
  onClick?: () => void;
  moreMenuItems?: DropdownItemProps[];
}

DataTable.Row = function Row({
  children,
  className,
  onClick,
  moreMenuItems,
  ...props
}: RowProps) {
  return (
    <tr
      className={classNames(
        "s-border-b s-border-structure-200 s-text-sm",
        onClick ? "s-cursor-pointer hover:s-bg-gray-50" : "",
        className || ""
      )}
      onClick={onClick ? onClick : undefined}
      {...props}
    >
      {children}
      {moreMenuItems && (
        <td className="s-w-1 s-cursor-pointer s-pl-1 s-text-element-600">
          <DropdownMenu className="s-flex">
            <DropdownMenu.Button
              onClick={(e) => {
                e.stopPropagation();
              }}
            >
              <Icon visual={MoreIcon} size="sm" />
            </DropdownMenu.Button>
            <DropdownMenu.Items origin="topRight" width={220}>
              {moreMenuItems?.map((item, index) => (
                <DropdownMenu.Item key={index} {...item} />
              ))}
            </DropdownMenu.Items>
          </DropdownMenu>
        </td>
      )}
    </tr>
  );
};

interface CellProps extends React.HTMLAttributes<HTMLTableCellElement> {
  children: ReactNode;
}

DataTable.Cell = function Cell({ children, className, ...props }: CellProps) {
  return (
    <td
      className={classNames(
        "s-whitespace-nowrap s-py-2 s-pl-0.5 s-text-element-800",
        className || ""
      )}
      {...props}
    >
      {children}
    </td>
  );
};

interface CellContentProps extends React.TdHTMLAttributes<HTMLDivElement> {
  avatarUrl?: string;
  icon?: React.ComponentType<{ className?: string }>;
  iconClassName?: string;
  roundedAvatar?: boolean;
  children?: ReactNode;
  description?: string;
}

DataTable.CellContent = function CellContent({
  children,
  className,
  avatarUrl,
  roundedAvatar,
  icon,
  iconClassName,
  description,
  ...props
}: CellContentProps) {
  return (
    <div className={classNames("s-flex s-py-2", className || "")} {...props}>
      {avatarUrl && (
        <Avatar
          visual={avatarUrl}
          size="xs"
          className="s-mr-2"
          isRounded={roundedAvatar ?? false}
        />
      )}
      {icon && (
        <Icon
          visual={icon}
          size="sm"
          className={classNames(
            "s-mr-2 s-text-element-600",
            iconClassName || ""
          )}
        />
      )}
      <div className="s-flex">
        <span className="s-text-sm s-text-element-800">{children}</span>
        {description && (
          <span className="s-pl-2 s-text-sm s-text-element-600">
            {description}
          </span>
        )}
      </div>
    </div>
  );
};

DataTable.Caption = function Caption({
  children,
  className,
  ...props
}: React.HTMLAttributes<HTMLTableCaptionElement>) {
  return (
    <caption className={className} {...props}>
      {children}
    </caption>
  );
};
